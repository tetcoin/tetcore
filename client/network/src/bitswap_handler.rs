// Copyright 2020 Parity Technologies (UK) Ltd.
// This file is part of Substrate.

// Substrate is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Substrate is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Substrate.  If not, see <http://www.gnu.org/licenses/>.

//! Helper for handling (i.e. answering) block requests from a remote peer via the
//! [`crate::request_responses::RequestResponsesBehaviour`].

use crate::chain::Client;
use crate::request_responses::{IncomingRequest, ProtocolConfig};
use crate::schema::bitswap::{Message as BitswapMessage,
	message::{wantlist::WantType, Block as MessageBlock, BlockPresenceType, BlockPresence},
};
use cid::{self, Version};
use codec::Encode;
use libp2p::core::PeerId;
use futures::channel::{mpsc, oneshot};
use futures::stream::StreamExt;
use log::debug;
use prost::Message;
use sp_runtime::traits::{Block as BlockT};
use std::sync::{Arc};
use std::time::Duration;
use unsigned_varint::{encode as varint_encode};

const LOG_TARGET: &str = "bitswap";

/// Prefix represents all metadata of a CID, without the actual content.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Prefix {
	/// The version of CID.
	pub version: Version,
	/// The codec of CID.
	pub codec: u64,
	/// The multihash type of CID.
	pub mh_type: u64,
	/// The multihash length of CID.
	pub mh_len: u8,
}

impl Prefix {
	/// Convert the prefix to encoded bytes.
	pub fn to_bytes(&self) -> Vec<u8> {
		let mut res = Vec::with_capacity(4);

		let mut buf = varint_encode::u64_buffer();
		let version = varint_encode::u64(self.version.into(), &mut buf);
		res.extend_from_slice(version);
		let mut buf = varint_encode::u64_buffer();
		let codec = varint_encode::u64(self.codec.into(), &mut buf);
		res.extend_from_slice(codec);
		let mut buf = varint_encode::u64_buffer();
		let mh_type = varint_encode::u64(self.mh_type.into(), &mut buf);
		res.extend_from_slice(mh_type);
		let mut buf = varint_encode::u64_buffer();
		let mh_len = varint_encode::u64(self.mh_len as u64, &mut buf);
		res.extend_from_slice(mh_len);

		res
	}
}


/// Handler for incoming block requests from a remote peer.
pub struct BitswapHandler<B> {
	client: Arc<dyn Client<B>>,
	request_receiver: mpsc::Receiver<IncomingRequest>,
}

impl <B: BlockT> BitswapHandler<B> {
	/// Create a new [`BitswapHandler`].
	pub fn new(client: Arc<dyn Client<B>>) -> (Self, ProtocolConfig) {
		// Rate of arrival multiplied with the waiting time in the queue equals the queue length.
		//
		// An average Polkadot sentry node serves less than 5 requests per second. The 95th percentile
		// serving a request is less than 2 second. Thus one would estimate the queue length to be
		// below 10.
		//
		// Choosing 20 as the queue length to give some additional buffer.
		let (tx, request_receiver) = mpsc::channel(20);

		let config = ProtocolConfig {
			name: "/ipfs/bitswap/1.2.0".into(),
			max_request_size: 1024,
			max_response_size: 16 * 1024 * 1024,
			request_timeout: Duration::from_secs(30),
			inbound_queue: Some(tx),
		};
		(Self { client, request_receiver }, config)
	}

	fn handle_request(
		&self,
		peer: &PeerId,
		payload: Vec<u8>,
		pending_response: oneshot::Sender<Vec<u8>>
	) -> Result<(), HandleRequestError> {
		let request: BitswapMessage = prost::Message::decode(payload.as_slice())?;
		log::info!("request: {:?}", request);
		let mut response = BitswapMessage {
			wantlist: None,
			blocks: Default::default(),
			payload: Default::default(),
			block_presences: Default::default(),
			pending_bytes: 0,
		};
		let wantlist = match request.wantlist {
			Some(wantlist) => wantlist,
			None => {
				debug!(
					target: LOG_TARGET,
					"Unexpected bitswap message from {}",
					peer,
				);
				return Ok(())
			}
		};
		for entry in wantlist.entries {
			let cid = cid::Cid::read_bytes(entry.block.as_slice())?;
			log::info!("requested cid: {:?}", cid);
			if cid.hash().code() != 0xb220 || cid.hash().size() != 32
			{
				log::info!(
					target: LOG_TARGET,
					"Ignoring unsupported cid {}: {}",
					peer, cid,
				);
				continue
			}
			let mut hash = B::Hash::default();
			hash.as_mut().copy_from_slice(&cid.hash().digest()[0..32]);
			log::info!("requested hash: {:?}", hash);
			let extrinsic = self.client.extrinsic(&hash)?;
			match extrinsic {
				Some(extrinsic) => {
					log::info!("requested hash: found");
					if entry.want_type == WantType::Block as i32 {
						let prefix = Prefix {
							version: cid.version(),
							codec: cid.codec(),
							mh_type: cid.hash().code(),
							mh_len: cid.hash().size(),
						};
						response.payload.push(MessageBlock {
							prefix: prefix.to_bytes(),
							data: extrinsic.encode(),
						});
					} else {
						log::info!("requested hash: not found");
						response.block_presences.push(BlockPresence {
							r#type: BlockPresenceType::Have as i32,
							cid: cid.to_bytes(),
						});
					}
				},
				None => {
					if entry.send_dont_have {
						response.block_presences.push(BlockPresence {
							r#type: BlockPresenceType::DontHave as i32,
							cid: cid.to_bytes(),
						});
					}
				}
			}
		}
		log::info!("response: {:?}", response);
		let mut data = Vec::with_capacity(response.encoded_len());
		response.encode(&mut data)?;

		pending_response.send(data).map_err(|_| HandleRequestError::SendResponse)?;
		Ok(())
	}

	/// Run [`BitswapHandler`].
	pub async fn run(mut self) {
		while let Some(request) = self.request_receiver.next().await {
			let IncomingRequest { peer, payload, pending_response } = request;

			match self.handle_request(&peer, payload, pending_response) {
				Ok(()) => debug!(target: LOG_TARGET, "Handled bitswap request from {}.", peer),
				Err(e) => debug!(
					target: LOG_TARGET,
					"Failed to handle bitswap request from {}: {}",
					peer, e,
				),
			}
		}
	}
}

#[derive(derive_more::Display, derive_more::From)]
enum HandleRequestError {
	#[display(fmt = "Failed to decode request: {}.", _0)]
	DecodeProto(prost::DecodeError),
	#[display(fmt = "Failed to encode response: {}.", _0)]
	EncodeProto(prost::EncodeError),
	Client(sp_blockchain::Error),
	BadCid(cid::Error),
	#[display(fmt = "Failed to send response.")]
	SendResponse,
}
