// This file is part of Tetcore.

// Copyright (C) 2018-2021 Parity Technologies (UK) Ltd.
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

//! Block Builder extensions for tests.

use tp_api::{ApiExt, ProvideRuntimeApi};
use tet_core::ChangesTrieConfiguration;
use tc_client_api::backend;
use tp_runtime::traits::HashFor;

use tc_block_builder::BlockBuilderApi;

/// Extension trait for test block builder.
pub trait BlockBuilderExt {
	/// Add transfer extrinsic to the block.
	fn push_transfer(&mut self, transfer: tetcore_test_runtime::Transfer) -> Result<(), tp_blockchain::Error>;
	/// Add storage change extrinsic to the block.
	fn push_storage_change(
		&mut self,
		key: Vec<u8>,
		value: Option<Vec<u8>>,
	) -> Result<(), tp_blockchain::Error>;
	/// Add changes trie configuration update extrinsic to the block.
	fn push_changes_trie_configuration_update(
		&mut self,
		new_config: Option<ChangesTrieConfiguration>,
	) -> Result<(), tp_blockchain::Error>;
}

impl<'a, A, B> BlockBuilderExt for tc_block_builder::BlockBuilder<'a, tetcore_test_runtime::Block, A, B> where
	A: ProvideRuntimeApi<tetcore_test_runtime::Block> + 'a,
	A::Api: BlockBuilderApi<tetcore_test_runtime::Block, Error = tp_blockchain::Error> +
		ApiExt<
			tetcore_test_runtime::Block,
			StateBackend = backend::StateBackendFor<B, tetcore_test_runtime::Block>
		>,
	B: backend::Backend<tetcore_test_runtime::Block>,
	// Rust bug: https://github.com/rust-lang/rust/issues/24159
	backend::StateBackendFor<B, tetcore_test_runtime::Block>:
		tp_api::StateBackend<HashFor<tetcore_test_runtime::Block>>,
{
	fn push_transfer(&mut self, transfer: tetcore_test_runtime::Transfer) -> Result<(), tp_blockchain::Error> {
		self.push(transfer.into_signed_tx())
	}

	fn push_storage_change(
		&mut self,
		key: Vec<u8>,
		value: Option<Vec<u8>>,
	) -> Result<(), tp_blockchain::Error> {
		self.push(tetcore_test_runtime::Extrinsic::StorageChange(key, value))
	}

	fn push_changes_trie_configuration_update(
		&mut self,
		new_config: Option<ChangesTrieConfiguration>,
	) -> Result<(), tp_blockchain::Error> {
		self.push(tetcore_test_runtime::Extrinsic::ChangesTrieConfigUpdate(new_config))
	}
}
