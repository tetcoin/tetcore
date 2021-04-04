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

//! Test utilities

#![cfg(test)]

use tp_runtime::testing::Header;
use tet_core::H256;
use fabric_support::parameter_types;
use crate::{self as noble_indices, Config};

type UncheckedExtrinsic = fabric_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = fabric_system::mocking::MockBlock<Test>;

fabric_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: fabric_system::{Module, Call, Config, Storage, Event<T>},
		Balances: noble_balances::{Module, Call, Storage, Config<T>, Event<T>},
		Indices: noble_indices::{Module, Call, Storage, Config<T>, Event<T>},
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub BlockWeights: fabric_system::limits::BlockWeights =
		fabric_system::limits::BlockWeights::simple_max(1024);
}

impl fabric_system::Config for Test {
	type BaseCallFilter = ();
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = ::tp_runtime::traits::BlakeTwo256;
	type AccountId = u64;
	type Lookup = Indices;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = BlockHashCount;
	type Version = ();
	type NobleInfo = NobleInfo;
	type AccountData = noble_balances::AccountData<u64>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ();
}

parameter_types! {
	pub const ExistentialDeposit: u64 = 1;
}

impl noble_balances::Config for Test {
	type MaxLocks = ();
	type Balance = u64;
	type DustRemoval = ();
	type Event = Event;
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
}

parameter_types! {
	pub const Deposit: u64 = 1;
}

impl Config for Test {
	type AccountIndex = u64;
	type Currency = Balances;
	type Deposit = Deposit;
	type Event = Event;
	type WeightInfo = ();
}

pub fn new_test_ext() -> tet_io::TestExternalities {
	let mut t = fabric_system::GenesisConfig::default().build_storage::<Test>().unwrap();
	noble_balances::GenesisConfig::<Test>{
		balances: vec![(1, 10), (2, 20), (3, 30), (4, 40), (5, 50), (6, 60)],
	}.assimilate_storage(&mut t).unwrap();
	t.into()
}
