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

// Benchmarks for Utility Noble

#![cfg(feature = "runtime-benchmarks")]

use super::*;
use fabric_system::{RawOrigin, EventRecord};
use fabric_benchmarking::{benchmarks, account, whitelisted_caller};

const SEED: u32 = 0;

fn assert_last_event<T: Config>(generic_event: <T as Config>::Event) {
	let events = fabric_system::Module::<T>::events();
	let system_event: <T as fabric_system::Config>::Event = generic_event.into();
	// compare to the last event record
	let EventRecord { event, .. } = &events[events.len() - 1];
	assert_eq!(event, &system_event);
}

benchmarks! {
	batch {
		let c in 0 .. 1000;
		let mut calls: Vec<<T as Config>::Call> = Vec::new();
		for i in 0 .. c {
			let call = fabric_system::Call::remark(vec![]).into();
			calls.push(call);
		}
		let caller = whitelisted_caller();
	}: _(RawOrigin::Signed(caller), calls)
	verify {
		assert_last_event::<T>(Event::BatchCompleted.into())
	}

	as_derivative {
		let caller = account("caller", SEED, SEED);
		let call = Box::new(fabric_system::Call::remark(vec![]).into());
		// Whitelist caller account from further DB operations.
		let caller_key = fabric_system::Account::<T>::hashed_key_for(&caller);
		fabric_benchmarking::benchmarking::add_to_whitelist(caller_key.into());
	}: _(RawOrigin::Signed(caller), SEED as u16, call)

	batch_all {
		let c in 0 .. 1000;
		let mut calls: Vec<<T as Config>::Call> = Vec::new();
		for i in 0 .. c {
			let call = fabric_system::Call::remark(vec![]).into();
			calls.push(call);
		}
		let caller = whitelisted_caller();
	}: _(RawOrigin::Signed(caller), calls)
	verify {
		assert_last_event::<T>(Event::BatchCompleted.into())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::tests::{new_test_ext, Test};
	use fabric_support::assert_ok;

	#[test]
	fn test_benchmarks() {
		new_test_ext().execute_with(|| {
			assert_ok!(test_benchmark_batch::<Test>());
			assert_ok!(test_benchmark_as_derivative::<Test>());
			assert_ok!(test_benchmark_batch_all::<Test>());
		});
	}
}