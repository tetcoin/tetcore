// This file is part of Tetcore.

// Copyright (C) 2019-2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! A collection of node-specific RPC methods.
//!
//! Since `tetcore` core functionality makes no assumptions
//! about the modules used inside the runtime, so do
//! RPC methods defined in `tc-rpc` crate.
//! It means that `client/rpc` can't have any methods that
//! need some strong assumptions about the particular runtime.
//!
//! The RPCs available in this crate however can make some assumptions
//! about how the runtime is constructed and what FRAME pallets
//! are part of it. Therefore all node-runtime-specific RPCs can
//! be placed here or imported from corresponding FRAME RPC definitions.

#![warn(missing_docs)]

use std::sync::Arc;

use tp_keystore::SyncCryptoStorePtr;
use node_primitives::{Block, BlockNumber, AccountId, Index, Balance, Hash};
use tc_consensus_babe::{Config, Epoch};
use tc_consensus_babe_rpc::BabeRpcHandler;
use tc_consensus_epochs::SharedEpochChanges;
use tc_finality_grandpa::{
	SharedVoterState, SharedAuthoritySet, FinalityProofProvider, GrandpaJustificationStream
};
use tc_finality_grandpa_rpc::GrandpaRpcHandler;
pub use tc_rpc_api::DenyUnsafe;
use tp_api::ProvideRuntimeApi;
use tp_block_builder::BlockBuilder;
use tp_blockchain::{Error as BlockChainError, HeaderMetadata, HeaderBackend};
use tp_consensus::SelectChain;
use tp_consensus_babe::BabeApi;
use tc_rpc::SubscriptionTaskExecutor;
use tp_transaction_pool::TransactionPool;
use tc_client_api::AuxStore;

/// Light client extra dependencies.
pub struct LightDeps<C, F, P> {
	/// The client instance to use.
	pub client: Arc<C>,
	/// Transaction pool instance.
	pub pool: Arc<P>,
	/// Remote access to the blockchain (async).
	pub remote_blockchain: Arc<dyn tc_client_api::light::RemoteBlockchain<Block>>,
	/// Fetcher instance.
	pub fetcher: Arc<F>,
}

/// Extra dependencies for BABE.
pub struct BabeDeps {
	/// BABE protocol config.
	pub babe_config: Config,
	/// BABE pending epoch changes.
	pub shared_epoch_changes: SharedEpochChanges<Block, Epoch>,
	/// The keystore that manages the keys of the node.
	pub keystore: SyncCryptoStorePtr,
}

/// Extra dependencies for GRANDPA
pub struct GrandpaDeps<B> {
	/// Voting round info.
	pub shared_voter_state: SharedVoterState,
	/// Authority set info.
	pub shared_authority_set: SharedAuthoritySet<Hash, BlockNumber>,
	/// Receives notifications about justification events from Grandpa.
	pub justification_stream: GrandpaJustificationStream<Block>,
	/// Executor to drive the subscription manager in the Grandpa RPC handler.
	pub subscription_executor: SubscriptionTaskExecutor,
	/// Finality proof provider.
	pub finality_provider: Arc<FinalityProofProvider<B, Block>>,
}

/// Full client dependencies.
pub struct FullDeps<C, P, SC, B> {
	/// The client instance to use.
	pub client: Arc<C>,
	/// Transaction pool instance.
	pub pool: Arc<P>,
	/// The SelectChain Strategy
	pub select_chain: SC,
	/// A copy of the chain spec.
	pub chain_spec: Box<dyn tc_chain_spec::ChainSpec>,
	/// Whether to deny unsafe calls
	pub deny_unsafe: DenyUnsafe,
	/// BABE specific dependencies.
	pub babe: BabeDeps,
	/// GRANDPA specific dependencies.
	pub grandpa: GrandpaDeps<B>,
}

/// A IO handler that uses all Full RPC extensions.
pub type IoHandler = tetsy_jsonrpc_core::IoHandler<tc_rpc::Metadata>;

/// Instantiate all Full RPC extensions.
pub fn create_full<C, P, SC, B>(
	deps: FullDeps<C, P, SC, B>,
) -> tetsy_jsonrpc_core::IoHandler<tc_rpc_api::Metadata> where
	C: ProvideRuntimeApi<Block> + HeaderBackend<Block> + AuxStore +
		HeaderMetadata<Block, Error=BlockChainError> + Sync + Send + 'static,
	C::Api: tetcore_frame_rpc_system::AccountNonceApi<Block, AccountId, Index>,
	C::Api: pallet_contracts_rpc::ContractsRuntimeApi<Block, AccountId, Balance, BlockNumber>,
	C::Api: pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance>,
	C::Api: BabeApi<Block>,
	C::Api: BlockBuilder<Block>,
	P: TransactionPool + 'static,
	SC: SelectChain<Block> +'static,
	B: tc_client_api::Backend<Block> + Send + Sync + 'static,
	B::State: tc_client_api::backend::StateBackend<tp_runtime::traits::HashFor<Block>>,
{
	use tetcore_frame_rpc_system::{FullSystem, SystemApi};
	use pallet_contracts_rpc::{Contracts, ContractsApi};
	use pallet_transaction_payment_rpc::{TransactionPayment, TransactionPaymentApi};

	let mut io = tetsy_jsonrpc_core::IoHandler::default();
	let FullDeps {
		client,
		pool,
		select_chain,
		chain_spec,
		deny_unsafe,
		babe,
		grandpa,
	} = deps;

	let BabeDeps {
		keystore,
		babe_config,
		shared_epoch_changes,
	} = babe;
	let GrandpaDeps {
		shared_voter_state,
		shared_authority_set,
		justification_stream,
		subscription_executor,
		finality_provider,
	} = grandpa;

	io.extend_with(
		SystemApi::to_delegate(FullSystem::new(client.clone(), pool, deny_unsafe))
	);
	// Making synchronous calls in light client freezes the browser currently,
	// more context: https://github.com/tetcoin/tetcore/pull/3480
	// These RPCs should use an asynchronous caller instead.
	io.extend_with(
		ContractsApi::to_delegate(Contracts::new(client.clone()))
	);
	io.extend_with(
		TransactionPaymentApi::to_delegate(TransactionPayment::new(client.clone()))
	);
	io.extend_with(
		tc_consensus_babe_rpc::BabeApi::to_delegate(
			BabeRpcHandler::new(
				client.clone(),
				shared_epoch_changes.clone(),
				keystore,
				babe_config,
				select_chain,
				deny_unsafe,
			),
		)
	);
	io.extend_with(
		tc_finality_grandpa_rpc::GrandpaApi::to_delegate(
			GrandpaRpcHandler::new(
				shared_authority_set.clone(),
				shared_voter_state,
				justification_stream,
				subscription_executor,
				finality_provider,
			)
		)
	);

	io.extend_with(
		tc_sync_state_rpc::SyncStateRpcApi::to_delegate(
			tc_sync_state_rpc::SyncStateRpcHandler::new(
				chain_spec,
				client,
				shared_authority_set,
				shared_epoch_changes,
				deny_unsafe,
			)
		)
	);

	io
}

/// Instantiate all Light RPC extensions.
pub fn create_light<C, P, M, F>(
	deps: LightDeps<C, F, P>,
) -> tetsy_jsonrpc_core::IoHandler<M> where
	C: tp_blockchain::HeaderBackend<Block>,
	C: Send + Sync + 'static,
	F: tc_client_api::light::Fetcher<Block> + 'static,
	P: TransactionPool + 'static,
	M: tetsy_jsonrpc_core::Metadata + Default,
{
	use tetcore_frame_rpc_system::{LightSystem, SystemApi};

	let LightDeps {
		client,
		pool,
		remote_blockchain,
		fetcher
	} = deps;
	let mut io = tetsy_jsonrpc_core::IoHandler::default();
	io.extend_with(
		SystemApi::<Hash, AccountId, Index>::to_delegate(LightSystem::new(client, remote_blockchain, fetcher, pool))
	);

	io
}
