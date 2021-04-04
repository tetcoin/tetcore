// This file is part of Tetcore.

// Copyright (C) 2020-2021 Parity Technologies (UK) Ltd.
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

//! Tests related to the noble version.

#![recursion_limit="128"]

use codec::{Decode, Encode};
use tp_runtime::{generic, traits::{BlakeTwo256, Verify}, BuildStorage};
use fabric_support::{
	traits::{NOBLE_VERSION_STORAGE_KEY_POSTFIX, NobleVersion, OnRuntimeUpgrade, GetNobleVersion},
	crate_to_noble_version, weights::Weight,
};
use tet_core::{H256, sr25519};

/// A version that we will check for in the tests
const SOME_TEST_VERSION: NobleVersion = NobleVersion { major: 3000, minor: 30, patch: 13 };

/// Checks that `on_runtime_upgrade` sets the latest noble version when being called without
/// being provided by the user.
mod module1 {
	pub trait Config: fabric_system::Config {}

	fabric_support::decl_module! {
		pub struct Module<T: Config> for enum Call where
			origin: <T as fabric_system::Config>::Origin,
		{}
	}
}

/// Checks that `on_runtime_upgrade` sets the latest noble version when being called and also
/// being provided by the user.
mod module2 {
	use super::*;

	pub trait Config<I=DefaultInstance>: fabric_system::Config {}

	fabric_support::decl_module! {
		pub struct Module<T: Config<I>, I: Instance=DefaultInstance> for enum Call where
			origin: <T as fabric_system::Config>::Origin,
		{
			fn on_runtime_upgrade() -> Weight {
				assert_eq!(crate_to_noble_version!(), Self::current_version());

				let version_key = NobleVersion::storage_key::<T::NobleInfo, Self>().unwrap();
				let version_value = tet_io::storage::get(&version_key);

				if version_value.is_some() {
					assert_eq!(SOME_TEST_VERSION, Self::storage_version().unwrap());
				} else {
					// As the storage version does not exist yet, it should be `None`.
					assert!(Self::storage_version().is_none());
				}

				0
			}
		}
	}

	fabric_support::decl_storage! {
		trait Store for Module<T: Config<I>, I: Instance=DefaultInstance> as Module2 {}
	}
}

#[fabric_support::noble]
mod noble3 {
	use fabric_support::noble_prelude::*;
	use fabric_system::noble_prelude::*;

	#[noble::config]
	pub trait Config: fabric_system::Config {
	}

	#[noble::noble]
	pub struct Noble<T>(PhantomData<T>);

	#[noble::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Noble<T> {
		fn on_runtime_upgrade() -> Weight {
			return 3;
		}
	}

	#[noble::call]
	impl<T: Config> Noble<T> {
	}
}

#[fabric_support::noble]
mod noble4 {
	use fabric_support::noble_prelude::*;
	use fabric_system::noble_prelude::*;

	#[noble::config]
	pub trait Config<I: 'static = ()>: fabric_system::Config {
	}

	#[noble::noble]
	pub struct Noble<T, I=()>(PhantomData<(T, I)>);

	#[noble::hooks]
	impl<T: Config<I>, I: 'static> Hooks<BlockNumberFor<T>> for Noble<T, I> {
		fn on_runtime_upgrade() -> Weight {
			return 3;
		}
	}

	#[noble::call]
	impl<T: Config<I>, I: 'static> Noble<T, I> {
	}
}

impl module1::Config for Runtime {}
impl module2::Config for Runtime {}
impl module2::Config<module2::Instance1> for Runtime {}
impl module2::Config<module2::Instance2> for Runtime {}

impl noble3::Config for Runtime {}
impl noble4::Config for Runtime {}
impl noble4::Config<noble4::Instance1> for Runtime {}
impl noble4::Config<noble4::Instance2> for Runtime {}

pub type Signature = sr25519::Signature;
pub type AccountId = <Signature as Verify>::Signer;
pub type BlockNumber = u64;
pub type Index = u64;

fabric_support::parameter_types!(
	pub const BlockHashCount: u32 = 250;
);

impl fabric_system::Config for Runtime {
	type BaseCallFilter = ();
	type Origin = Origin;
	type Index = u64;
	type BlockNumber = BlockNumber;
	type Call = Call;
	type Hash = H256;
	type Hashing = tp_runtime::traits::BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = tp_runtime::traits::IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = BlockHashCount;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Version = ();
	type NobleInfo = NobleInfo;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ();
}

fabric_support::construct_runtime!(
	pub enum Runtime where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic
	{
		System: fabric_system::{Module, Call, Event<T>},
		Module1: module1::{Module, Call},
		Module2: module2::{Module, Call},
		Module2_1: module2::<Instance1>::{Module, Call},
		Module2_2: module2::<Instance2>::{Module, Call},
		Noble3: noble3::{Module, Call},
		Noble4: noble4::{Module, Call},
		Noble4_1: noble4::<Instance1>::{Module, Call},
		Noble4_2: noble4::<Instance2>::{Module, Call},
	}
);

pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
pub type Block = generic::Block<Header, UncheckedExtrinsic>;
pub type UncheckedExtrinsic = generic::UncheckedExtrinsic<u32, Call, Signature, ()>;

/// Returns the storage key for `NobleVersion` for the given `noble`.
fn get_noble_version_storage_key_for_noble(noble: &str) -> [u8; 32] {
	let noble_name = tet_io::hashing::twox_128(noble.as_bytes());
	let postfix = tet_io::hashing::twox_128(NOBLE_VERSION_STORAGE_KEY_POSTFIX);

	let mut final_key = [0u8; 32];
	final_key[..16].copy_from_slice(&noble_name);
	final_key[16..].copy_from_slice(&postfix);

	final_key
}

/// Checks the version of the given `noble`.
///
/// It is expected that the noble version can be found in the storage and equals the
/// current crate version.
fn check_noble_version(noble: &str) {
	let key = get_noble_version_storage_key_for_noble(noble);
	let value = tet_io::storage::get(&key).expect("Noble version exists");
	let version = NobleVersion::decode(&mut &value[..])
		.expect("Noble version is encoded correctly");

	assert_eq!(crate_to_noble_version!(), version);
}

#[test]
fn on_runtime_upgrade_sets_the_noble_versions_in_storage() {
	tet_io::TestExternalities::new_empty().execute_with(|| {
		AllModules::on_runtime_upgrade();

		check_noble_version("Module1");
		check_noble_version("Module2");
		check_noble_version("Module2_1");
		check_noble_version("Module2_2");
		check_noble_version("Noble3");
		check_noble_version("Noble4");
		check_noble_version("Noble4_1");
		check_noble_version("Noble4_2");
	});
}

#[test]
fn on_runtime_upgrade_overwrites_old_version() {
	tet_io::TestExternalities::new_empty().execute_with(|| {
		let key = get_noble_version_storage_key_for_noble("Module2");
		tet_io::storage::set(&key, &SOME_TEST_VERSION.encode());

		AllModules::on_runtime_upgrade();

		check_noble_version("Module1");
		check_noble_version("Module2");
		check_noble_version("Module2_1");
		check_noble_version("Module2_2");
		check_noble_version("Noble3");
		check_noble_version("Noble4");
		check_noble_version("Noble4_1");
		check_noble_version("Noble4_2");
	});
}

#[test]
fn genesis_init_puts_noble_version_into_storage() {
	let storage = GenesisConfig::default().build_storage().expect("Builds genesis storage");

	tet_io::TestExternalities::new(storage).execute_with(|| {
		check_noble_version("Module1");
		check_noble_version("Module2");
		check_noble_version("Module2_1");
		check_noble_version("Module2_2");
		check_noble_version("Noble3");
		check_noble_version("Noble4");
		check_noble_version("Noble4_1");
		check_noble_version("Noble4_2");

		let system_version = System::storage_version().expect("System version should be set");
		assert_eq!(System::current_version(), system_version);
	});
}
