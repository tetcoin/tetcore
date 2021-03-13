// This file is part of Tetcore.

// Copyright (C) 2020-2021 Parity Technologies (UK) Ltd.
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

use serde::{Serialize, Deserialize};

use tc_finality_grandpa::FinalityProofProvider;
use tp_runtime::traits::{Block as BlockT, NumberFor};

#[derive(Serialize, Deserialize)]
pub struct EncodedFinalityProof(pub tet_core::Bytes);

/// Local trait mainly to allow mocking in tests.
pub trait RpcFinalityProofProvider<Block: BlockT> {
	/// Prove finality for the given block number by returning a Justification for the last block of
	/// the authority set.
	fn rpc_prove_finality(
		&self,
		block: NumberFor<Block>,
	) -> Result<Option<EncodedFinalityProof>, tc_finality_grandpa::FinalityProofError>;
}

impl<B, Block> RpcFinalityProofProvider<Block> for FinalityProofProvider<B, Block>
where
	Block: BlockT,
	NumberFor<Block>: tetsy_finality_grandpa::BlockNumberOps,
	B: tc_client_api::backend::Backend<Block> + Send + Sync + 'static,
{
	fn rpc_prove_finality(
		&self,
		block: NumberFor<Block>,
	) -> Result<Option<EncodedFinalityProof>, tc_finality_grandpa::FinalityProofError> {
		self.prove_finality(block)
			.map(|x| x.map(|y| EncodedFinalityProof(y.into())))
	}
}
