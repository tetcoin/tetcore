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

use fabric_support::{
	weights::{DispatchInfo, DispatchClass, Pays, GetDispatchInfo},
	traits::{
		GetCallName, GetNobleVersion, OnInitialize, OnFinalize, OnRuntimeUpgrade, OnGenesis,
	},
	dispatch::UnfilteredDispatchable,
	storage::unhashed,
};
use tp_runtime::DispatchError;
use tet_io::{TestExternalities, hashing::{twox_64, twox_128, blake2_128}};

#[fabric_support::noble]
pub mod noble {
	use tetcore_std::any::TypeId;
	use fabric_support::noble_prelude::*;
	use fabric_system::noble_prelude::*;

	type BalanceOf<T, I> = <T as Config<I>>::Balance;

	#[noble::config]
	pub trait Config<I: 'static = ()>: fabric_system::Config {
		#[noble::constant]
		type MyGetParam: Get<u32>;
		type Balance: Parameter + Default;
		type Event: From<Event<Self, I>> + IsType<<Self as fabric_system::Config>::Event>;
	}

	#[noble::noble]
	#[noble::generate_store(pub(crate) trait Store)]
	pub struct Noble<T, I = ()>(PhantomData<(T, I)>);

	#[noble::hooks]
	impl<T: Config<I>, I: 'static> Hooks<BlockNumberFor<T>> for Noble<T, I> {
		fn on_initialize(_: BlockNumberFor<T>) -> Weight {
			if TypeId::of::<I>() == TypeId::of::<()>() {
				Self::deposit_event(Event::Something(10));
				10
			} else {
				Self::deposit_event(Event::Something(11));
				11
			}
		}
		fn on_finalize(_: BlockNumberFor<T>) {
			if TypeId::of::<I>() == TypeId::of::<()>() {
				Self::deposit_event(Event::Something(20));
			} else {
				Self::deposit_event(Event::Something(21));
			}
		}
		fn on_runtime_upgrade() -> Weight {
			if TypeId::of::<I>() == TypeId::of::<()>() {
				Self::deposit_event(Event::Something(30));
				30
			} else {
				Self::deposit_event(Event::Something(31));
				31
			}
		}
		fn integrity_test() {
		}
	}

	#[noble::call]
	impl<T: Config<I>, I: 'static> Noble<T, I> {
		/// Doc comment put in metadata
		#[noble::weight(Weight::from(*_foo))]
		fn foo(origin: OriginFor<T>, #[noble::compact] _foo: u32) -> DispatchResultWithPostInfo {
			let _ = origin;
			Self::deposit_event(Event::Something(3));
			Ok(().into())
		}

		/// Doc comment put in metadata
		#[noble::weight(1)]
		#[fabric_support::transactional]
		fn foo_transactional(
			origin: OriginFor<T>,
			#[noble::compact] _foo: u32
		) -> DispatchResultWithPostInfo {
			let _ = origin;
			Ok(().into())
		}
	}


	#[noble::error]
	pub enum Error<T, I = ()> {
		/// doc comment put into metadata
		InsufficientProposersBalance,
	}

	#[noble::event]
	#[noble::metadata(BalanceOf<T, I> = "Balance", u32 = "Other")]
	#[noble::generate_deposit(fn deposit_event)]
	pub enum Event<T: Config<I>, I: 'static = ()> {
		/// doc comment put in metadata
		Proposed(<T as fabric_system::Config>::AccountId),
		/// doc
		Spending(BalanceOf<T, I>),
		Something(u32),
	}

	#[noble::storage]
	pub type Value<T, I = ()> = StorageValue<_, u32>;

	#[noble::storage]
	pub type Map<T, I = ()> = StorageMap<_, Blake2_128Concat, u8, u16>;

	#[noble::storage]
	pub type Map2<T, I = ()> = StorageMap<_, Twox64Concat, u16, u32>;

	#[noble::storage]
	pub type DoubleMap<T, I = ()> =
		StorageDoubleMap<_, Blake2_128Concat, u8, Twox64Concat, u16, u32>;

	#[noble::storage]
	pub type DoubleMap2<T, I = ()> =
		StorageDoubleMap<_, Twox64Concat, u16, Blake2_128Concat, u32, u64>;

	#[noble::genesis_config]
	#[derive(Default)]
	pub struct GenesisConfig {
		_myfield: u32,
	}

	#[noble::genesis_build]
	impl<T: Config<I>, I:'static> GenesisBuild<T, I> for GenesisConfig {
		fn build(&self) {}
	}

	#[noble::origin]
	#[derive(EqNoBound, RuntimeDebugNoBound, CloneNoBound, PartialEqNoBound, Encode, Decode)]
	pub struct Origin<T, I = ()>(PhantomData<(T, I)>);

	#[noble::validate_unsigned]
	impl<T: Config<I>, I: 'static> ValidateUnsigned for Noble<T, I> {
		type Call = Call<T, I>;
		fn validate_unsigned(
			_source: TransactionSource,
			_call: &Self::Call
		) -> TransactionValidity {
			Err(TransactionValidityError::Invalid(InvalidTransaction::Call))
		}
	}

	#[noble::inherent]
	impl<T: Config<I>, I: 'static> ProvideInherent for Noble<T, I> {
		type Call = Call<T, I>;
		type Error = InherentError;

		const INHERENT_IDENTIFIER: InherentIdentifier = INHERENT_IDENTIFIER;

		fn create_inherent(_data: &InherentData) -> Option<Self::Call> {
			unimplemented!();
		}
	}

	#[derive(codec::Encode, tp_runtime::RuntimeDebug)]
	#[cfg_attr(feature = "std", derive(codec::Decode))]
	pub enum InherentError {
	}

	impl tp_inherents::IsFatalError for InherentError {
		fn is_fatal_error(&self) -> bool {
			unimplemented!();
		}
	}

	pub const INHERENT_IDENTIFIER: tp_inherents::InherentIdentifier = *b"testpall";
}

// Test that a instantiable noble with a generic genesis_config is correctly handled
#[fabric_support::noble]
pub mod noble2 {
	use fabric_support::noble_prelude::*;
	use fabric_system::noble_prelude::*;

	#[noble::config]
	pub trait Config<I: 'static = ()>: fabric_system::Config {
		type Event: From<Event<Self, I>> + IsType<<Self as fabric_system::Config>::Event>;
	}

	#[noble::noble]
	#[noble::generate_store(pub(crate) trait Store)]
	pub struct Noble<T, I = ()>(PhantomData<(T, I)>);

	#[noble::hooks]
	impl<T: Config<I>, I: 'static> Hooks<BlockNumberFor<T>> for Noble<T, I> {}

	#[noble::call]
	impl<T: Config<I>, I: 'static> Noble<T, I> {}

	#[noble::event]
	pub enum Event<T: Config<I>, I: 'static = ()> {
		/// Something
		Something(u32),
	}

	#[noble::genesis_config]
	pub struct GenesisConfig<T: Config<I>, I: 'static = ()> {
		phantom: PhantomData<(T, I)>,
	}

	impl<T: Config<I>, I: 'static> Default for GenesisConfig<T, I> {
		fn default() -> Self {
			GenesisConfig {
				phantom: Default::default(),
			}
		}
	}

	#[noble::genesis_build]
	impl<T: Config<I>, I: 'static> GenesisBuild<T, I> for GenesisConfig<T, I> {
		fn build(&self) {}
	}
}

fabric_support::parameter_types!(
	pub const MyGetParam: u32= 10;
	pub const BlockHashCount: u32 = 250;
);

impl fabric_system::Config for Runtime {
	type BaseCallFilter = ();
	type Origin = Origin;
	type Index = u64;
	type BlockNumber = u32;
	type Call = Call;
	type Hash = tp_runtime::testing::H256;
	type Hashing = tp_runtime::traits::BlakeTwo256;
	type AccountId = u64;
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
impl noble::Config for Runtime {
	type Event = Event;
	type MyGetParam= MyGetParam;
	type Balance = u64;
}
impl noble::Config<noble::Instance1> for Runtime {
	type Event = Event;
	type MyGetParam= MyGetParam;
	type Balance = u64;
}
impl noble2::Config for Runtime {
	type Event = Event;
}
impl noble2::Config<noble::Instance1> for Runtime {
	type Event = Event;
}

pub type Header = tp_runtime::generic::Header<u32, tp_runtime::traits::BlakeTwo256>;
pub type Block = tp_runtime::generic::Block<Header, UncheckedExtrinsic>;
pub type UncheckedExtrinsic = tp_runtime::generic::UncheckedExtrinsic<u32, Call, (), ()>;

fabric_support::construct_runtime!(
	pub enum Runtime where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic
	{
		System: fabric_system::{Module, Call, Event<T>},
		Example: noble::{Module, Call, Event<T>, Config, Storage, Inherent, Origin<T>, ValidateUnsigned},
		Instance1Example: noble::<Instance1>::{
			Module, Call, Event<T>, Config, Storage, Inherent, Origin<T>, ValidateUnsigned
		},
		Example2: noble2::{Module, Call, Event<T>, Config<T>, Storage},
		Instance1Example2: noble2::<Instance1>::{Module, Call, Event<T>, Config<T>, Storage},
	}
);

#[test]
fn call_expand() {
	let call_foo = noble::Call::<Runtime>::foo(3);
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
		noble::Call::<Runtime>::get_call_names(),
		&["foo", "foo_transactional"],
	);

	let call_foo = noble::Call::<Runtime, noble::Instance1>::foo(3);
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
		noble::Call::<Runtime, noble::Instance1>::get_call_names(),
		&["foo", "foo_transactional"],
	);
}

#[test]
fn error_expand() {
	assert_eq!(
		format!("{:?}", noble::Error::<Runtime>::InsufficientProposersBalance),
		String::from("InsufficientProposersBalance"),
	);
	assert_eq!(
		<&'static str>::from(noble::Error::<Runtime>::InsufficientProposersBalance),
		"InsufficientProposersBalance",
	);
	assert_eq!(
		DispatchError::from(noble::Error::<Runtime>::InsufficientProposersBalance),
		DispatchError::Module {
			index: 1,
			error: 0,
			message: Some("InsufficientProposersBalance"),
		},
	);

	assert_eq!(
		format!("{:?}", noble::Error::<Runtime, noble::Instance1>::InsufficientProposersBalance),
		String::from("InsufficientProposersBalance"),
	);
	assert_eq!(
		<&'static str>::from(noble::Error::<Runtime, noble::Instance1>::InsufficientProposersBalance),
		"InsufficientProposersBalance",
	);
	assert_eq!(
		DispatchError::from(noble::Error::<Runtime, noble::Instance1>::InsufficientProposersBalance),
		DispatchError::Module {
			index: 2,
			error: 0,
			message: Some("InsufficientProposersBalance"),
		},
	);
}

#[test]
fn instance_expand() {
	// assert same type
	let _: noble::__InherentHiddenInstance = ();
}

#[test]
fn noble_expand_deposit_event() {
	TestExternalities::default().execute_with(|| {
		fabric_system::Module::<Runtime>::set_block_number(1);
		noble::Call::<Runtime>::foo(3).dispatch_bypass_filter(None.into()).unwrap();
		assert_eq!(
			fabric_system::Module::<Runtime>::events()[0].event,
			Event::noble(noble::Event::Something(3)),
		);
	});

	TestExternalities::default().execute_with(|| {
		fabric_system::Module::<Runtime>::set_block_number(1);
		noble::Call::<Runtime, noble::Instance1>::foo(3).dispatch_bypass_filter(None.into()).unwrap();
		assert_eq!(
			fabric_system::Module::<Runtime>::events()[0].event,
			Event::noble_Instance1(noble::Event::Something(3)),
		);
	});
}

#[test]
fn storage_expand() {
	use fabric_support::noble_prelude::*;
	use fabric_support::StoragePrefixedMap;

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
		<noble::Value<Runtime>>::put(1);
		let k = [twox_128(b"Example"), twox_128(b"Value")].concat();
		assert_eq!(unhashed::get::<u32>(&k), Some(1u32));

		<noble::Map<Runtime>>::insert(1, 2);
		let mut k = [twox_128(b"Example"), twox_128(b"Map")].concat();
		k.extend(1u8.using_encoded(blake2_128_concat));
		assert_eq!(unhashed::get::<u16>(&k), Some(2u16));
		assert_eq!(&k[..32], &<noble::Map<Runtime>>::final_prefix());

		<noble::Map2<Runtime>>::insert(1, 2);
		let mut k = [twox_128(b"Example"), twox_128(b"Map2")].concat();
		k.extend(1u16.using_encoded(twox_64_concat));
		assert_eq!(unhashed::get::<u32>(&k), Some(2u32));
		assert_eq!(&k[..32], &<noble::Map2<Runtime>>::final_prefix());

		<noble::DoubleMap<Runtime>>::insert(&1, &2, &3);
		let mut k = [twox_128(b"Example"), twox_128(b"DoubleMap")].concat();
		k.extend(1u8.using_encoded(blake2_128_concat));
		k.extend(2u16.using_encoded(twox_64_concat));
		assert_eq!(unhashed::get::<u32>(&k), Some(3u32));
		assert_eq!(&k[..32], &<noble::DoubleMap<Runtime>>::final_prefix());

		<noble::DoubleMap2<Runtime>>::insert(&1, &2, &3);
		let mut k = [twox_128(b"Example"), twox_128(b"DoubleMap2")].concat();
		k.extend(1u16.using_encoded(twox_64_concat));
		k.extend(2u32.using_encoded(blake2_128_concat));
		assert_eq!(unhashed::get::<u64>(&k), Some(3u64));
		assert_eq!(&k[..32], &<noble::DoubleMap2<Runtime>>::final_prefix());
	});

	TestExternalities::default().execute_with(|| {
		<noble::Value<Runtime, noble::Instance1>>::put(1);
		let k = [twox_128(b"Instance1Example"), twox_128(b"Value")].concat();
		assert_eq!(unhashed::get::<u32>(&k), Some(1u32));

		<noble::Map<Runtime, noble::Instance1>>::insert(1, 2);
		let mut k = [twox_128(b"Instance1Example"), twox_128(b"Map")].concat();
		k.extend(1u8.using_encoded(blake2_128_concat));
		assert_eq!(unhashed::get::<u16>(&k), Some(2u16));
		assert_eq!(&k[..32], &<noble::Map<Runtime, noble::Instance1>>::final_prefix());

		<noble::Map2<Runtime, noble::Instance1>>::insert(1, 2);
		let mut k = [twox_128(b"Instance1Example"), twox_128(b"Map2")].concat();
		k.extend(1u16.using_encoded(twox_64_concat));
		assert_eq!(unhashed::get::<u32>(&k), Some(2u32));
		assert_eq!(&k[..32], &<noble::Map2<Runtime, noble::Instance1>>::final_prefix());

		<noble::DoubleMap<Runtime, noble::Instance1>>::insert(&1, &2, &3);
		let mut k = [twox_128(b"Instance1Example"), twox_128(b"DoubleMap")].concat();
		k.extend(1u8.using_encoded(blake2_128_concat));
		k.extend(2u16.using_encoded(twox_64_concat));
		assert_eq!(unhashed::get::<u32>(&k), Some(3u32));
		assert_eq!(&k[..32], &<noble::DoubleMap<Runtime, noble::Instance1>>::final_prefix());

		<noble::DoubleMap2<Runtime, noble::Instance1>>::insert(&1, &2, &3);
		let mut k = [twox_128(b"Instance1Example"), twox_128(b"DoubleMap2")].concat();
		k.extend(1u16.using_encoded(twox_64_concat));
		k.extend(2u32.using_encoded(blake2_128_concat));
		assert_eq!(unhashed::get::<u64>(&k), Some(3u64));
		assert_eq!(&k[..32], &<noble::DoubleMap2<Runtime, noble::Instance1>>::final_prefix());
	});
}

#[test]
fn noble_hooks_expand() {
	TestExternalities::default().execute_with(|| {
		fabric_system::Module::<Runtime>::set_block_number(1);

		assert_eq!(AllModules::on_initialize(1), 21);
		AllModules::on_finalize(1);

		assert_eq!(noble::Noble::<Runtime>::storage_version(), None);
		assert_eq!(noble::Noble::<Runtime, noble::Instance1>::storage_version(), None);
		assert_eq!(AllModules::on_runtime_upgrade(), 61);
		assert_eq!(
			noble::Noble::<Runtime>::storage_version(),
			Some(noble::Noble::<Runtime>::current_version()),
		);
		assert_eq!(
			noble::Noble::<Runtime, noble::Instance1>::storage_version(),
			Some(noble::Noble::<Runtime, noble::Instance1>::current_version()),
		);

		// The order is indeed reversed due to https://github.com/tetcoin/tetcore/issues/6280
		assert_eq!(
			fabric_system::Module::<Runtime>::events()[0].event,
			Event::noble_Instance1(noble::Event::Something(11)),
		);
		assert_eq!(
			fabric_system::Module::<Runtime>::events()[1].event,
			Event::noble(noble::Event::Something(10)),
		);
		assert_eq!(
			fabric_system::Module::<Runtime>::events()[2].event,
			Event::noble_Instance1(noble::Event::Something(21)),
		);
		assert_eq!(
			fabric_system::Module::<Runtime>::events()[3].event,
			Event::noble(noble::Event::Something(20)),
		);
		assert_eq!(
			fabric_system::Module::<Runtime>::events()[4].event,
			Event::noble_Instance1(noble::Event::Something(31)),
		);
		assert_eq!(
			fabric_system::Module::<Runtime>::events()[5].event,
			Event::noble(noble::Event::Something(30)),
		);
	})
}

#[test]
fn noble_on_genesis() {
	TestExternalities::default().execute_with(|| {
		assert_eq!(noble::Noble::<Runtime>::storage_version(), None);
		noble::Noble::<Runtime>::on_genesis();
		assert_eq!(
			noble::Noble::<Runtime>::storage_version(),
			Some(noble::Noble::<Runtime>::current_version()),
		);

		assert_eq!(noble::Noble::<Runtime, noble::Instance1>::storage_version(), None);
		noble::Noble::<Runtime, noble::Instance1>::on_genesis();
		assert_eq!(
			noble::Noble::<Runtime, noble::Instance1>::storage_version(),
			Some(noble::Noble::<Runtime, noble::Instance1>::current_version()),
		);
	})
}

#[test]
fn metadata() {
	use fabric_metadata::*;
	use codec::{Decode, Encode};

	let expected_noble_metadata = ModuleMetadata {
		index: 1,
		name: DecodeDifferent::Decoded("Example".to_string()),
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
				arguments: DecodeDifferent::Decoded(vec!["<T as fabric_system::Config>::AccountId".to_string()]),
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

	let mut expected_noble_instance1_metadata = expected_noble_metadata.clone();
	expected_noble_instance1_metadata.name = DecodeDifferent::Decoded("Instance1Example".to_string());
	expected_noble_instance1_metadata.index = 2;
	match expected_noble_instance1_metadata.storage {
		Some(DecodeDifferent::Decoded(ref mut storage_meta)) => {
			storage_meta.prefix = DecodeDifferent::Decoded("Instance1Example".to_string());
		},
		_ => unreachable!(),
	}


	let metadata = match Runtime::metadata().1 {
		RuntimeMetadata::V12(metadata) => metadata,
		_ => panic!("metadata has been bump, test needs to be updated"),
	};

	let modules_metadata = match metadata.modules {
		DecodeDifferent::Encode(modules_metadata) => modules_metadata,
		_ => unreachable!(),
	};

	let noble_metadata = ModuleMetadata::decode(&mut &modules_metadata[1].encode()[..]).unwrap();
	let noble_instance1_metadata =
		ModuleMetadata::decode(&mut &modules_metadata[2].encode()[..]).unwrap();

	pretty_assertions::assert_eq!(noble_metadata, expected_noble_metadata);
	pretty_assertions::assert_eq!(noble_instance1_metadata, expected_noble_instance1_metadata);
}
