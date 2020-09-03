// This file is part of Substrate.

// Copyright (C) 2020 Parity Technologies (UK) Ltd.
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

use frame_support::{
	weights::{DispatchInfo, DispatchClass, Pays, GetDispatchInfo},
	traits::{GetCallName, Instance, OnInitialize, OnFinalize, OnRuntimeUpgrade},
	dispatch::UnfilteredDispatchable,
	storage::unhashed,
};
use sp_runtime::{traits::Block as _, DispatchError};
use sp_io::{TestExternalities, hashing::{twox_64, twox_128, blake2_128}};

#[frame_support::pallet(Example)]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	type BalanceOf<T> = <T as Trait>::Balance;

	#[pallet::trait_]
	pub trait Trait: frame_system::Trait {
		#[pallet::const_]
		type MyGetParam: Get<u32>;
		type Balance: Parameter + Default;
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Trait>::Event>;
	}

	#[pallet::module]
	#[pallet::generate(fn deposit_event)]
	pub struct Module<T>(PhantomData<T>);

	#[pallet::module_interface]
	impl<T: Trait> ModuleInterface<BlockNumberFor<T>> for Module<T> {
		fn on_initialize(_: BlockNumberFor<T>) -> Weight {
			Self::deposit_event(Event::Something(10));
			10
		}
		fn on_finalize(_: BlockNumberFor<T>) {
			Self::deposit_event(Event::Something(20));
		}
		fn on_runtime_upgrade() -> Weight {
			Self::deposit_event(Event::Something(30));
			30
		}
		fn integrity_test() {
		}
	}

	#[pallet::call]
	impl<T: Trait> Call for Module<T> {
		/// Doc comment put in metadata
		#[pallet::weight(Weight::from(*_foo))]
		fn foo(origin: OriginFor<T>, #[pallet::compact] _foo: u32) -> DispatchResultWithPostInfo {
			let _ = origin;
			Self::deposit_event(Event::Something(3));
			Ok(().into())
		}

		/// Doc comment put in metadata
		#[pallet::weight(1)]
		#[frame_support::transactional]
		fn foo_transactional(origin: OriginFor<T>, #[pallet::compact] _foo: u32) -> DispatchResultWithPostInfo {
			let _ = origin;
			Ok(().into())
		}
	}


	#[pallet::error]
	pub enum Error<T> {
		/// doc comment put into metadata
		InsufficientProposersBalance,
	}

	#[pallet::event]
	#[pallet::metadata(BalanceOf<T> = Balance, u32 = Other)]
	pub enum Event<T: Trait> {
		/// doc comment put in metadata
		Proposed(<T as frame_system::Trait>::AccountId),
		/// doc
		Spending(BalanceOf<T>),
		Something(u32),
	}

	#[pallet::storage]
	pub type Value = StorageValueType<_, u32>;

	#[pallet::storage]
	pub type Map = StorageMapType<_, Blake2_128Concat, u8, u16>;

	#[pallet::storage]
	pub type Map2 = StorageMapType<_, Twox64Concat, u16, u32>;

	#[pallet::storage]
	pub type DoubleMap = StorageDoubleMapType<_, Blake2_128Concat, u8, Twox64Concat, u16, u32>;

	#[pallet::storage]
	pub type DoubleMap2 = StorageDoubleMapType<_, Twox64Concat, u16, Blake2_128Concat, u32, u64>;

	#[pallet::genesis_config]
	#[derive(Default)]
	pub struct GenesisConfig {
		_myfield: u32,
	}

	#[pallet::genesis_build]
	impl<T: Trait> GenesisBuilder<T> for GenesisConfig {
		fn build(&self) {}
	}

	#[pallet::origin]
	#[derive(EqNoBound, DebugStripped, CloneNoBound, PartialEqNoBound, Encode, Decode)]
	pub struct Origin<T>(PhantomData<T>);

	#[pallet::validate_unsigned]
	impl<T: Trait> ValidateUnsigned for Module<T> {
		type Call = Call<T>;
		fn validate_unsigned(
			_source: TransactionSource,
			_call: &Self::Call
		) -> TransactionValidity {
			Err(TransactionValidityError::Invalid(InvalidTransaction::Call))
		}
	}

	#[pallet::inherent]
	impl<T: Trait> ProvideInherent for Module<T> {
		type Call = Call<T>;
		type Error = InherentError;

		const INHERENT_IDENTIFIER: InherentIdentifier = INHERENT_IDENTIFIER;

		fn create_inherent(_data: &InherentData) -> Option<Self::Call> {
			unimplemented!();
		}
	}

	#[derive(codec::Encode, sp_runtime::RuntimeDebug)]
	#[cfg_attr(feature = "std", derive(codec::Decode))]
	pub enum InherentError {
	}

	impl sp_inherents::IsFatalError for InherentError {
		fn is_fatal_error(&self) -> bool {
			unimplemented!();
		}
	}

	pub const INHERENT_IDENTIFIER: sp_inherents::InherentIdentifier = *b"testpall";
}

frame_support::parameter_types!(
	pub const MyGetParam: u32= 10;
	pub const BlockHashCount: u32 = 250;
	pub const MaximumBlockWeight: frame_support::weights::Weight = 1024;
	pub const MaximumBlockLength: u32 = 2 * 1024;
	pub const AvailableBlockRatio: sp_runtime::Perbill = sp_runtime::Perbill::one();
);

impl frame_system::Trait for Runtime {
	type BaseCallFilter = ();
	type Origin = Origin;
	type Index = u64;
	type BlockNumber = u32;
	type Call = Call;
	type Hash = sp_runtime::testing::H256;
	type Hashing = sp_runtime::traits::BlakeTwo256;
	type AccountId = u64;
	type Lookup = sp_runtime::traits::IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = BlockHashCount;
	type MaximumBlockWeight = MaximumBlockWeight;
	type DbWeight = frame_support::weights::constants::RocksDbWeight;
	type BlockExecutionWeight = ();
	type ExtrinsicBaseWeight = ();
	type MaximumExtrinsicWeight = MaximumBlockWeight;
	type AvailableBlockRatio = AvailableBlockRatio;
	type MaximumBlockLength = MaximumBlockLength;
	type Version = ();
	type ModuleToIndex = ModuleToIndex;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
}
impl pallet::Trait for Runtime {
	type Event = Event;
	type MyGetParam= MyGetParam;
	type Balance = u64;
}

pub type Header = sp_runtime::generic::Header<u32, sp_runtime::traits::BlakeTwo256>;
pub type Block = sp_runtime::generic::Block<Header, UncheckedExtrinsic>;
pub type UncheckedExtrinsic = sp_runtime::generic::UncheckedExtrinsic<u32, Call, (), ()>;

frame_support::construct_runtime!(
	pub enum Runtime where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic
	{
		System: frame_system::{Module, Call, Event<T>},
		Pallet: pallet::{Module, Call, Event<T>, Config, Storage, Inherent, Origin<T>, ValidateUnsigned},
	}
);

#[test]
fn call_expand() {
	let call_foo = pallet::Call::<Runtime>::foo(3);
	assert_eq!(
		call_foo.get_dispatch_info(),
		DispatchInfo {
			weight: 3,
			class: DispatchClass::Normal,
			pays_fee: Pays::Yes,
		}
	);
	assert_eq!(call_foo.get_call_name(), "foo");
	assert_eq!(
		pallet::Call::<Runtime>::get_call_names(),
		&["foo", "foo_transactional"],
	);
}

#[test]
fn error_expand() {
	assert_eq!(
		format!("{:?}", pallet::Error::<Runtime>::InsufficientProposersBalance),
		String::from("InsufficientProposersBalance"),
	);
	assert_eq!(
		<&'static str>::from(pallet::Error::<Runtime>::InsufficientProposersBalance),
		"InsufficientProposersBalance",
	);
	assert_eq!(
		DispatchError::from(pallet::Error::<Runtime>::InsufficientProposersBalance),
		DispatchError::Module {
			index: 1,
			error: 0,
			message: Some("InsufficientProposersBalance"),
		},
	);
}

#[test]
fn instance_expand() {
	assert_eq!(pallet::__InherentHiddenInstance::PREFIX, "Example");
}

#[test]
fn module_expand_deposit_event() {
	TestExternalities::default().execute_with(|| {
		frame_system::Module::<Runtime>::set_block_number(1);
		pallet::Call::<Runtime>::foo(3).dispatch_bypass_filter(None.into()).unwrap();
		assert_eq!(
			frame_system::Module::<Runtime>::events()[0].event,
			Event::pallet(pallet::Event::Something(3)),
		);
	})
}

#[test]
fn storage_expand() {
	use frame_support::pallet_prelude::*;

	fn twox_64_concat(d: &[u8]) -> Vec<u8> {
		let mut v = twox_64(d).to_vec();
		v.extend_from_slice(d);
		v
	}

	fn blake2_128_concat(d: &[u8]) -> Vec<u8> {
		let mut v = blake2_128(d).to_vec();
		v.extend_from_slice(d);
		v
	}

	TestExternalities::default().execute_with(|| {
		pallet::Value::put(1);
		let k = [twox_128(b"Example"), twox_128(b"Value")].concat();
		assert_eq!(unhashed::get::<u32>(&k), Some(1u32));

		pallet::Map::insert(1, 2);
		let mut k = [twox_128(b"Example"), twox_128(b"Map")].concat();
		k.extend(1u8.using_encoded(blake2_128_concat));
		assert_eq!(unhashed::get::<u16>(&k), Some(2u16));
		assert_eq!(&k[..32], &<pallet::Map>::final_prefix());

		pallet::Map2::insert(1, 2);
		let mut k = [twox_128(b"Example"), twox_128(b"Map2")].concat();
		k.extend(1u16.using_encoded(twox_64_concat));
		assert_eq!(unhashed::get::<u32>(&k), Some(2u32));
		assert_eq!(&k[..32], &<pallet::Map2>::final_prefix());

		pallet::DoubleMap::insert(&1, &2, &3);
		let mut k = [twox_128(b"Example"), twox_128(b"DoubleMap")].concat();
		k.extend(1u8.using_encoded(blake2_128_concat));
		k.extend(2u16.using_encoded(twox_64_concat));
		assert_eq!(unhashed::get::<u32>(&k), Some(3u32));
		assert_eq!(&k[..32], &<pallet::DoubleMap>::final_prefix());

		pallet::DoubleMap2::insert(&1, &2, &3);
		let mut k = [twox_128(b"Example"), twox_128(b"DoubleMap2")].concat();
		k.extend(1u16.using_encoded(twox_64_concat));
		k.extend(2u32.using_encoded(blake2_128_concat));
		assert_eq!(unhashed::get::<u64>(&k), Some(3u64));
		assert_eq!(&k[..32], &<pallet::DoubleMap2>::final_prefix());
	})
}

#[test]
fn module_interface_expand() {
	TestExternalities::default().execute_with(|| {
		frame_system::Module::<Runtime>::set_block_number(1);

		assert_eq!(AllModules::on_initialize(1), 10);
		AllModules::on_finalize(1);
		assert_eq!(AllModules::on_runtime_upgrade(), 30);

		assert_eq!(
			frame_system::Module::<Runtime>::events()[0].event,
			Event::pallet(pallet::Event::Something(10)),
		);
		assert_eq!(
			frame_system::Module::<Runtime>::events()[1].event,
			Event::pallet(pallet::Event::Something(20)),
		);
		assert_eq!(
			frame_system::Module::<Runtime>::events()[2].event,
			Event::pallet(pallet::Event::Something(30)),
		);
	})
}

#[test]
fn metadata() {
	use frame_metadata::*;
	use codec::{Decode, Encode};

	let expected_pallet_metadata = ModuleMetadata {
		name: DecodeDifferent::Decoded("Pallet".to_string()),
		storage: Some(DecodeDifferent::Decoded(StorageMetadata {
			prefix: DecodeDifferent::Decoded("Example".to_string()),
			entries: DecodeDifferent::Decoded(vec![
				StorageEntryMetadata {
					name: DecodeDifferent::Decoded("Value".to_string()),
					modifier: StorageEntryModifier::Optional,
					ty: StorageEntryType::Plain(DecodeDifferent::Decoded("u32".to_string())),
					default: DecodeDifferent::Decoded(vec![0]),
					documentation: DecodeDifferent::Decoded(vec![]),
				},
				StorageEntryMetadata {
					name: DecodeDifferent::Decoded("Map".to_string()),
					modifier: StorageEntryModifier::Optional,
					ty: StorageEntryType::Map {
						key: DecodeDifferent::Decoded("u8".to_string()),
						value: DecodeDifferent::Decoded("u16".to_string()),
						hasher: StorageHasher::Blake2_128Concat,
						unused: false,
					},
					default: DecodeDifferent::Decoded(vec![0]),
					documentation: DecodeDifferent::Decoded(vec![]),
				},
				StorageEntryMetadata {
					name: DecodeDifferent::Decoded("Map2".to_string()),
					modifier: StorageEntryModifier::Optional,
					ty: StorageEntryType::Map {
						key: DecodeDifferent::Decoded("u16".to_string()),
						value: DecodeDifferent::Decoded("u32".to_string()),
						hasher: StorageHasher::Twox64Concat,
						unused: false,
					},
					default: DecodeDifferent::Decoded(vec![0]),
					documentation: DecodeDifferent::Decoded(vec![]),
				},
				StorageEntryMetadata {
					name: DecodeDifferent::Decoded("DoubleMap".to_string()),
					modifier: StorageEntryModifier::Optional,
					ty: StorageEntryType::DoubleMap {
						value: DecodeDifferent::Decoded("u32".to_string()),
						key1: DecodeDifferent::Decoded("u8".to_string()),
						key2: DecodeDifferent::Decoded("u16".to_string()),
						hasher: StorageHasher::Blake2_128Concat,
						key2_hasher: StorageHasher::Twox64Concat,
					},
					default: DecodeDifferent::Decoded(vec![0]),
					documentation: DecodeDifferent::Decoded(vec![]),
				},
				StorageEntryMetadata {
					name: DecodeDifferent::Decoded("DoubleMap2".to_string()),
					modifier: StorageEntryModifier::Optional,
					ty: StorageEntryType::DoubleMap {
						value: DecodeDifferent::Decoded("u64".to_string()),
						key1: DecodeDifferent::Decoded("u16".to_string()),
						key2: DecodeDifferent::Decoded("u32".to_string()),
						hasher: StorageHasher::Twox64Concat,
						key2_hasher: StorageHasher::Blake2_128Concat,
					},
					default: DecodeDifferent::Decoded(vec![0]),
					documentation: DecodeDifferent::Decoded(vec![]),
				},
			]),
		})),
		calls: Some(DecodeDifferent::Decoded(vec![
			FunctionMetadata {
				name: DecodeDifferent::Decoded("foo".to_string()),
				arguments: DecodeDifferent::Decoded(vec![
					FunctionArgumentMetadata {
						name: DecodeDifferent::Decoded("_foo".to_string()),
						ty: DecodeDifferent::Decoded("Compact<u32>".to_string()),
					}
				]),
				documentation: DecodeDifferent::Decoded(vec![
					" Doc comment put in metadata".to_string(),
				]),
			},
			FunctionMetadata {
				name: DecodeDifferent::Decoded("foo_transactional".to_string()),
				arguments: DecodeDifferent::Decoded(vec![
					FunctionArgumentMetadata {
						name: DecodeDifferent::Decoded("_foo".to_string()),
						ty: DecodeDifferent::Decoded("Compact<u32>".to_string()),
					}
				]),
				documentation: DecodeDifferent::Decoded(vec![
					" Doc comment put in metadata".to_string(),
				]),
			},
		])),
		event: Some(DecodeDifferent::Decoded(vec![
			EventMetadata {
				name: DecodeDifferent::Decoded("Proposed".to_string()),
				arguments: DecodeDifferent::Decoded(vec!["AccountId".to_string()]),
				documentation: DecodeDifferent::Decoded(vec![
					" doc comment put in metadata".to_string()
				]),
			},
			EventMetadata {
				name: DecodeDifferent::Decoded("Spending".to_string()),
				arguments: DecodeDifferent::Decoded(vec!["Balance".to_string()]),
				documentation: DecodeDifferent::Decoded(vec![
					" doc".to_string()
				]),
			},
			EventMetadata {
				name: DecodeDifferent::Decoded("Something".to_string()),
				arguments: DecodeDifferent::Decoded(vec!["Other".to_string()]),
				documentation: DecodeDifferent::Decoded(vec![]),
			},
		])),
		constants: DecodeDifferent::Decoded(vec![
			ModuleConstantMetadata {
				name: DecodeDifferent::Decoded("MyGetParam".to_string()),
				ty: DecodeDifferent::Decoded("u32".to_string()),
				value: DecodeDifferent::Decoded(vec![10, 0, 0, 0]),
				documentation: DecodeDifferent::Decoded(vec![]),
			},
		]),
		errors: DecodeDifferent::Decoded(vec![
			ErrorMetadata {
				name: DecodeDifferent::Decoded("InsufficientProposersBalance".to_string()),
				documentation: DecodeDifferent::Decoded(vec![
					" doc comment put into metadata".to_string(),
				]),
			},
		]),
	};

	let metadata = match Runtime::metadata().1 {
		RuntimeMetadata::V11(metadata) => metadata,
		_ => panic!("metadata has been bump, test needs to be updated"),
	};

	let modules_metadata = match metadata.modules {
		DecodeDifferent::Encode(modules_metadata) => modules_metadata,
		_ => unreachable!(),
	};

	let pallet_metadata = ModuleMetadata::decode(&mut &modules_metadata[1].encode()[..]).unwrap();

	pretty_assertions::assert_eq!(pallet_metadata, expected_pallet_metadata);
}
