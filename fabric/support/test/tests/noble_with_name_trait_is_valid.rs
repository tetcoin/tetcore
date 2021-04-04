// This file is part of Tetcore.

// Copyright (C) 2017-2021 Parity Technologies (UK) Ltd.
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

pub trait Trait: fabric_system::Config {
	type Balance: fabric_support::dispatch::Parameter;
	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as fabric_system::Config>::Event>;
}

fabric_support::decl_storage! {
	trait Store for Module<T: Trait> as Example {
		Dummy get(fn dummy) config(): Option<u32>;
	}
}

fabric_support::decl_event!(
	pub enum Event<T> where B = <T as Trait>::Balance {
		Dummy(B),
	}
);

fabric_support::decl_error!(
	pub enum Error for Module<T: Trait> {
		Dummy,
	}
);

fabric_support::decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event() = default;
		type Error = Error<T>;
		const Foo: u32 = u32::max_value();

		#[weight = 0]
		fn accumulate_dummy(origin, increase_by: T::Balance) {
			unimplemented!();
		}

		fn on_initialize(_n: T::BlockNumber) -> fabric_support::weights::Weight {
			0
		}
	}
}

impl<T: Trait> tp_runtime::traits::ValidateUnsigned for Module<T> {
	type Call = Call<T>;

	fn validate_unsigned(
		_source: tp_runtime::transaction_validity::TransactionSource,
		_call: &Self::Call,
	) -> tp_runtime::transaction_validity::TransactionValidity {
		unimplemented!();
	}
}

pub const INHERENT_IDENTIFIER: tp_inherents::InherentIdentifier = *b"12345678";

impl<T: Trait> tp_inherents::ProvideInherent for Module<T> {
	type Call = Call<T>;
	type Error = tp_inherents::MakeFatalError<tp_inherents::Error>;
	const INHERENT_IDENTIFIER: tp_inherents::InherentIdentifier = INHERENT_IDENTIFIER;

	fn create_inherent(_data: &tp_inherents::InherentData) -> Option<Self::Call> {
		unimplemented!();
	}

	fn check_inherent(_: &Self::Call, _: &tp_inherents::InherentData) -> std::result::Result<(), Self::Error> {
		unimplemented!();
	}
}

#[cfg(test)]
mod tests {
	use crate as noble_test;

	use fabric_support::parameter_types;

	type SignedExtra = (
		fabric_system::CheckEra<Runtime>,
		fabric_system::CheckNonce<Runtime>,
		fabric_system::CheckWeight<Runtime>,
	);
	type TestBlock = tp_runtime::generic::Block<TestHeader, TestUncheckedExtrinsic>;
	type TestHeader = tp_runtime::generic::Header<u64, tp_runtime::traits::BlakeTwo256>;
	type TestUncheckedExtrinsic = tp_runtime::generic::UncheckedExtrinsic<
		<Runtime as fabric_system::Config>::AccountId,
		<Runtime as fabric_system::Config>::Call,
		(),
		SignedExtra,
	>;

	fabric_support::construct_runtime!(
		pub enum Runtime where
			Block = TestBlock,
			NodeBlock = TestBlock,
			UncheckedExtrinsic = TestUncheckedExtrinsic
		{
			System: fabric_system::{Module, Call, Config, Storage, Event<T>},
			NobleTest: noble_test::{Module, Call, Storage, Event<T>, Config, ValidateUnsigned, Inherent},
		}
	);

	parameter_types! {
		pub const BlockHashCount: u64 = 250;
	}

	impl fabric_system::Config for Runtime {
		type BaseCallFilter = ();
		type Origin = Origin;
		type Index = u64;
		type BlockNumber = u64;
		type Hash = tet_core::H256;
		type Call = Call;
		type Hashing = tp_runtime::traits::BlakeTwo256;
		type AccountId = u64;
		type Lookup = tp_runtime::traits::IdentityLookup<Self::AccountId>;
		type Header = TestHeader;
		type Event = ();
		type BlockHashCount = BlockHashCount;
		type DbWeight = ();
		type BlockWeights = ();
		type BlockLength = ();
		type Version = ();
		type NobleInfo = ();
		type AccountData = ();
		type OnNewAccount = ();
		type OnKilledAccount = ();
		type SystemWeightInfo = ();
		type SS58Prefix = ();
	}

	impl noble_test::Trait for Runtime {
		type Balance = u32;
		type Event = ();
	}
}
