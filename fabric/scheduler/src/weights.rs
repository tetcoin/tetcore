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

//! Weights for noble_scheduler
//! THIS FILE WAS AUTO-GENERATED USING THE TETCORE BENCHMARK CLI VERSION 2.0.0
//! DATE: 2020-10-27, STEPS: [50, ], REPEAT: 20, LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// target/release/tetcore
// benchmark
// --chain=dev
// --steps=50
// --repeat=20
// --noble=noble_scheduler
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./fabric/scheduler/src/weights.rs
// --template=./.maintain/fabric-weight-template.hbs


#![allow(unused_parens)]
#![allow(unused_imports)]

use fabric_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use tetcore_std::marker::PhantomData;

/// Weight functions needed for noble_scheduler.
pub trait WeightInfo {
	fn schedule(s: u32, ) -> Weight;
	fn cancel(s: u32, ) -> Weight;
	fn schedule_named(s: u32, ) -> Weight;
	fn cancel_named(s: u32, ) -> Weight;

}

/// Weights for noble_scheduler using the Tetcore node and recommended hardware.
pub struct TetcoreWeight<T>(PhantomData<T>);
impl<T: fabric_system::Config> WeightInfo for TetcoreWeight<T> {
	fn schedule(s: u32, ) -> Weight {
		(35_029_000 as Weight)
			.saturating_add((77_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))

	}
	fn cancel(s: u32, ) -> Weight {
		(31_419_000 as Weight)
			.saturating_add((4_015_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))

	}
	fn schedule_named(s: u32, ) -> Weight {
		(44_752_000 as Weight)
			.saturating_add((123_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))

	}
	fn cancel_named(s: u32, ) -> Weight {
		(35_712_000 as Weight)
			.saturating_add((4_008_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))

	}

}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn schedule(s: u32, ) -> Weight {
		(35_029_000 as Weight)
			.saturating_add((77_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))

	}
	fn cancel(s: u32, ) -> Weight {
		(31_419_000 as Weight)
			.saturating_add((4_015_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))

	}
	fn schedule_named(s: u32, ) -> Weight {
		(44_752_000 as Weight)
			.saturating_add((123_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))

	}
	fn cancel_named(s: u32, ) -> Weight {
		(35_712_000 as Weight)
			.saturating_add((4_008_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))

	}

}
