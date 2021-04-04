// This file is part of Tetcore.

// Copyright (C) 2020-2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Weights for noble_timestamp
//! THIS FILE WAS AUTO-GENERATED USING THE TETCORE BENCHMARK CLI VERSION 2.0.0
//! DATE: 2020-10-27, STEPS: [50, ], REPEAT: 20, LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// target/release/tetcore
// benchmark
// --chain=dev
// --steps=50
// --repeat=20
// --noble=noble_timestamp
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./fabric/timestamp/src/weights.rs
// --template=./.maintain/fabric-weight-template.hbs


#![allow(unused_parens)]
#![allow(unused_imports)]

use fabric_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use tetcore_std::marker::PhantomData;

/// Weight functions needed for noble_timestamp.
pub trait WeightInfo {
	fn set() -> Weight;
	fn on_finalize() -> Weight;

}

/// Weights for noble_timestamp using the Tetcore node and recommended hardware.
pub struct TetcoreWeight<T>(PhantomData<T>);
impl<T: fabric_system::Config> WeightInfo for TetcoreWeight<T> {
	fn set() -> Weight {
		(11_650_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))

	}
	fn on_finalize() -> Weight {
		(6_681_000 as Weight)

	}

}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn set() -> Weight {
		(11_650_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))

	}
	fn on_finalize() -> Weight {
		(6_681_000 as Weight)

	}

}
