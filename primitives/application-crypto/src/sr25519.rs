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

//! Sr25519 crypto types.

use crate::{RuntimePublic, KeyTypeId};

use tetcore_std::vec::Vec;

pub use sp_core::sr25519::*;

mod app {
	use sp_core::testing::SR25519;

	crate::app_crypto!(super, SR25519);

	impl crate::traits::BoundToRuntimeAppPublic for Public {
		type Public = Self;
	}
}

pub use app::{Public as AppPublic, Signature as AppSignature};
#[cfg(feature = "full_crypto")]
pub use app::Pair as AppPair;

impl RuntimePublic for Public {
	type Signature = Signature;

	fn all(key_type: KeyTypeId) -> crate::Vec<Self> {
		sp_io::crypto::sr25519_public_keys(key_type)
	}

	fn generate_pair(key_type: KeyTypeId, seed: Option<Vec<u8>>) -> Self {
		sp_io::crypto::sr25519_generate(key_type, seed)
	}

	fn sign<M: AsRef<[u8]>>(&self, key_type: KeyTypeId, msg: &M) -> Option<Self::Signature> {
		sp_io::crypto::sr25519_sign(key_type, self, msg.as_ref())
	}

	fn verify<M: AsRef<[u8]>>(&self, msg: &M, signature: &Self::Signature) -> bool {
		sp_io::crypto::sr25519_verify(&signature, msg.as_ref(), self)
	}

	fn to_raw_vec(&self) -> Vec<u8> {
		sp_core::crypto::Public::to_raw_vec(self)
	}
}
