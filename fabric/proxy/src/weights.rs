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

//! Weights for noble_proxy
//! THIS FILE WAS AUTO-GENERATED USING THE TETCORE BENCHMARK CLI VERSION 2.0.0
//! DATE: 2020-10-27, STEPS: [50, ], REPEAT: 20, LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// target/release/tetcore
// benchmark
// --chain=dev
// --steps=50
// --repeat=20
// --noble=noble_proxy
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./fabric/proxy/src/weights.rs
// --template=./.maintain/fabric-weight-template.hbs


#![allow(unused_parens)]
#![allow(unused_imports)]

use fabric_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use tetcore_std::marker::PhantomData;

/// Weight functions needed for noble_proxy.
pub trait WeightInfo {
	fn proxy(p: u32, ) -> Weight;
	fn proxy_announced(a: u32, p: u32, ) -> Weight;
	fn remove_announcement(a: u32, p: u32, ) -> Weight;
	fn reject_announcement(a: u32, p: u32, ) -> Weight;
	fn announce(a: u32, p: u32, ) -> Weight;
	fn add_proxy(p: u32, ) -> Weight;
	fn remove_proxy(p: u32, ) -> Weight;
	fn remove_proxies(p: u32, ) -> Weight;
	fn anonymous(p: u32, ) -> Weight;
	fn kill_anonymous(p: u32, ) -> Weight;

}

/// Weights for noble_proxy using the Tetcore node and recommended hardware.
pub struct TetcoreWeight<T>(PhantomData<T>);
impl<T: fabric_system::Config> WeightInfo for TetcoreWeight<T> {
	fn proxy(p: u32, ) -> Weight {
		(32_194_000 as Weight)
			.saturating_add((215_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))

	}
	fn proxy_announced(a: u32, p: u32, ) -> Weight {
		(67_490_000 as Weight)
			.saturating_add((859_000 as Weight).saturating_mul(a as Weight))
			.saturating_add((215_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))

	}
	fn remove_announcement(a: u32, p: u32, ) -> Weight {
		(40_768_000 as Weight)
			.saturating_add((882_000 as Weight).saturating_mul(a as Weight))
			.saturating_add((122_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))

	}
	fn reject_announcement(a: u32, p: u32, ) -> Weight {
		(42_742_000 as Weight)
			.saturating_add((852_000 as Weight).saturating_mul(a as Weight))
			.saturating_add((22_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))

	}
	fn announce(a: u32, p: u32, ) -> Weight {
		(67_967_000 as Weight)
			.saturating_add((737_000 as Weight).saturating_mul(a as Weight))
			.saturating_add((213_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))

	}
	fn add_proxy(p: u32, ) -> Weight {
		(45_245_000 as Weight)
			.saturating_add((240_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))

	}
	fn remove_proxy(p: u32, ) -> Weight {
		(40_742_000 as Weight)
			.saturating_add((272_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))

	}
	fn remove_proxies(p: u32, ) -> Weight {
		(39_070_000 as Weight)
			.saturating_add((214_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))

	}
	fn anonymous(p: u32, ) -> Weight {
		(64_851_000 as Weight)
			.saturating_add((37_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))

	}
	fn kill_anonymous(p: u32, ) -> Weight {
		(41_831_000 as Weight)
			.saturating_add((207_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))

	}

}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn proxy(p: u32, ) -> Weight {
		(32_194_000 as Weight)
			.saturating_add((215_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))

	}
	fn proxy_announced(a: u32, p: u32, ) -> Weight {
		(67_490_000 as Weight)
			.saturating_add((859_000 as Weight).saturating_mul(a as Weight))
			.saturating_add((215_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))

	}
	fn remove_announcement(a: u32, p: u32, ) -> Weight {
		(40_768_000 as Weight)
			.saturating_add((882_000 as Weight).saturating_mul(a as Weight))
			.saturating_add((122_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))

	}
	fn reject_announcement(a: u32, p: u32, ) -> Weight {
		(42_742_000 as Weight)
			.saturating_add((852_000 as Weight).saturating_mul(a as Weight))
			.saturating_add((22_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))

	}
	fn announce(a: u32, p: u32, ) -> Weight {
		(67_967_000 as Weight)
			.saturating_add((737_000 as Weight).saturating_mul(a as Weight))
			.saturating_add((213_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))

	}
	fn add_proxy(p: u32, ) -> Weight {
		(45_245_000 as Weight)
			.saturating_add((240_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))

	}
	fn remove_proxy(p: u32, ) -> Weight {
		(40_742_000 as Weight)
			.saturating_add((272_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))

	}
	fn remove_proxies(p: u32, ) -> Weight {
		(39_070_000 as Weight)
			.saturating_add((214_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))

	}
	fn anonymous(p: u32, ) -> Weight {
		(64_851_000 as Weight)
			.saturating_add((37_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))

	}
	fn kill_anonymous(p: u32, ) -> Weight {
		(41_831_000 as Weight)
			.saturating_add((207_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))

	}

}
