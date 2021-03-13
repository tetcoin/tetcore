// This file is part of Tetcore.

// Copyright (C) 2017-2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Light client utilities.

use std::sync::Arc;

use tc_executor::RuntimeInfo;
use tet_core::traits::{CodeExecutor, SpawnNamed};
use tp_runtime::BuildStorage;
use tp_runtime::traits::{Block as BlockT, HashFor};
use tp_blockchain::Result as ClientResult;
use prometheus_endpoint::Registry;

use super::{call_executor::LocalCallExecutor, client::{Client, ClientConfig}};
use tc_client_api::light::Storage as BlockchainStorage;
use tc_light::{Backend, GenesisCallExecutor};


/// Create an instance of light client.
pub fn new_light<B, S, RA, E>(
	backend: Arc<Backend<S, HashFor<B>>>,
	genesis_storage: &dyn BuildStorage,
	code_executor: E,
	spawn_handle: Box<dyn SpawnNamed>,
	prometheus_registry: Option<Registry>,
) -> ClientResult<
		Client<
			Backend<S, HashFor<B>>,
			GenesisCallExecutor<
				Backend<S, HashFor<B>>,
				LocalCallExecutor<Backend<S, HashFor<B>>, E>
			>,
			B,
			RA
		>
	>
	where
		B: BlockT,
		S: BlockchainStorage<B> + 'static,
		E: CodeExecutor + RuntimeInfo + Clone + 'static,
{
	let local_executor = LocalCallExecutor::new(
		backend.clone(),
		code_executor,
		spawn_handle.clone(),
		ClientConfig::default()
	)?;
	let executor = GenesisCallExecutor::new(backend.clone(), local_executor);
	Client::new(
		backend,
		executor,
		genesis_storage,
		Default::default(),
		Default::default(),
		Default::default(),
		prometheus_registry,
		ClientConfig::default(),
	)
}
