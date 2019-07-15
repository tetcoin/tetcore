// Copyright 2019 Parity Technologies (UK) Ltd.
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

//! HTTP request manager for offchain workers.

use std::collections::HashMap;

use futures::{Future, Sink};
use hyper_tls::HttpsConnector;
use hyper::{
	Request,
	http::{self, HttpTryFrom, request::Builder},
	client::{Client, HttpConnector, ResponseFuture},
};

use primitives::offchain::{
	HttpRequestId, Timestamp,
};

#[derive(Debug)]
pub enum Error {
	Http(http::Error),
	IncorrectStatus(HttpRequestId, String),
	MissingRequest(HttpRequestId),
	Unavailable,
}

pub type Result<T> = std::result::Result<T, Error>;

impl<T: Into<http::Error>> From<T> for Error {
	fn from(err: T) -> Self {
		Error::Http(err.into())
	}
}

#[derive(Debug)]
enum RequestStatus {
	Building(Builder),
	StreamingBody(ResponseFuture, hyper::body::Sender),
	Completed(u32),
	Invalid,
}

impl RequestStatus {
	fn is_building(&self) -> bool {
		match *self  {
			RequestStatus::Building(..) => true,
			_ => false,
		}
	}
}

#[derive(Debug)]
pub struct HttpRequestManager {
	// TODO [ToDr] This should be shared between multiple offchain workers.
	client: Client<HttpsConnector<HttpConnector>, hyper::Body>,
	requests: HashMap<HttpRequestId, RequestStatus>,
	next_id: u16,
}

impl HttpRequestManager {
	pub fn new() -> Self {
		let https = HttpsConnector::new(4).unwrap();
		let client = Client::builder().build(https);

		HttpRequestManager {
			client,
			requests: Default::default(),
			next_id: Default::default(),
		}
	}

	pub fn request_start(
		&mut self,
		method: &str,
		uri: &str,
		_meta: &[u8],
	) -> Result<HttpRequestId> {
		let method = hyper::Method::try_from(method)?;
		let uri = hyper::Uri::try_from(uri)?;

		let mut builder = Request::builder();
		builder
			.method(method)
			.uri(uri);

		// TODO [ToDr] Mechanism to re-sue request ids when we saturate u16
		let id = HttpRequestId(
			self.next_id.checked_add(1).ok_or(Error::Unavailable)?
		);
		self.requests.insert(id.into(), RequestStatus::Building(builder));

		Ok(id.into())
	}

	pub fn request_add_header(
		&mut self,
		request_id: HttpRequestId,
		name: &str,
		value: &str
	) -> Result<()> {
		match self.requests.get_mut(&request_id) {
			Some(RequestStatus::Building(ref mut builder)) => {
				builder.header(name, value);
				Ok(())
			},
			Some(status) => Err(Error::IncorrectStatus(request_id, format!("{:?}", status))),
			None => Err(Error::MissingRequest(request_id)),
		}
	}

	pub fn request_write_body(
		&mut self,
		request_id: HttpRequestId,
		chunk: &[u8],
		deadline: Option<Timestamp>
	) -> Result<()> {
		let write_chunk = move |future: ResponseFuture, sender: hyper::body::Sender| {
			sender.send(chunk.to_vec().into()).wait()
		};
		let status = self.requests
			.get_mut(&request_id)
			.ok_or_else(|| Error::MissingRequest(request_id))?;

		let (future, sender) = match std::mem::replace(status, RequestStatus::Invalid) {
			RequestStatus::Building(builder) => {

			}
		}
	}

}
