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
		GetCallName, OnInitialize, OnFinalize, OnRuntimeUpgrade, GetNobleVersion, OnGenesis,
	},
	dispatch::{UnfilteredDispatchable, Parameter},
	storage::unhashed,
};
use tp_runtime::DispatchError;
use tet_io::{TestExternalities, hashing::{twox_64, twox_128, blake2_128}};

pub struct SomeType1;
impl From<SomeType1> for u64 { fn from(_t: SomeType1) -> Self { 0u64 } }

pub struct SomeType2;
impl From<SomeType2> for u64 { fn from(_t: SomeType2) -> Self { 100u64 } }

pub struct SomeType3;
impl From<SomeType3> for u64 { fn from(_t: SomeType3) -> Self { 0u64 } }

pub struct SomeType4;
impl From<SomeType4> for u64 { fn from(_t: SomeType4) -> Self { 0u64 } }

pub struct SomeType5;
impl From<SomeType5> for u64 { fn from(_t: SomeType5) -> Self { 0u64 } }

pub struct SomeType6;
impl From<SomeType6> for u64 { fn from(_t: SomeType6) -> Self { 0u64 } }

pub struct SomeType7;
impl From<SomeType7> for u64 { fn from(_t: SomeType7) -> Self { 0u64 } }

pub trait SomeAssociation1 { type _1: Parameter; }
impl SomeAssociation1 for u64 { type _1 = u64; }

pub trait SomeAssociation2 { type _2: Parameter; }
impl SomeAssociation2 for u64 { type _2 = u64; }

#[fabric_support::noble]
pub mod noble {
	use super::{
		SomeType1, SomeType2, SomeType3, SomeType4, SomeType5, SomeType6, SomeType7,
		SomeAssociation1, SomeAssociation2,
	};
	use fabric_support::noble_prelude::*;
	use fabric_system::noble_prelude::*;

	type BalanceOf<T> = <T as Config>::Balance;

	#[noble::config]
	pub trait Config: fabric_system::Config
	where <Self as fabric_system::Config>::AccountId: From<SomeType1> + SomeAssociation1,
	{
		/// Some comment
		/// Some comment
		#[noble::constant]
		type MyGetParam: Get<u32>;

		/// Some comment
		/// Some comment
		#[noble::constant]
		type MyGetParam2: Get<u32>;

		#[noble::constant]
		type MyGetParam3: Get<<Self::AccountId as SomeAssociation1>::_1>;

		type Balance: Parameter + Default;

		type Event: From<Event<Self>> + IsType<<Self as fabric_system::Config>::Event>;
	}

	#[noble::extra_constants]
	impl<T: Config> Noble<T>
	where T::AccountId: From<SomeType1> + SomeAssociation1 + From<SomeType2>,
	{
		/// Some doc
		/// Some doc
		fn some_extra() -> T::AccountId { SomeType2.into() }

		/// Some doc
		fn some_extra_extra() -> T::AccountId { SomeType1.into() }
	}

	#[noble::noble]
	#[noble::generate_store(pub(crate) trait Store)]
	pub struct Noble<T>(PhantomData<T>);

	#[noble::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Noble<T>
	where T::AccountId: From<SomeType2> + From<SomeType1> + SomeAssociation1,
	{
		fn on_initialize(_: BlockNumberFor<T>) -> Weight {
			T::AccountId::from(SomeType1); // Test for where clause
			T::AccountId::from(SomeType2); // Test for where clause
			Self::deposit_event(Event::Something(10));
			10
		}
		fn on_finalize(_: BlockNumberFor<T>) {
			T::AccountId::from(SomeType1); // Test for where clause
			T::AccountId::from(SomeType2); // Test for where clause
			Self::deposit_event(Event::Something(20));
		}
		fn on_runtime_upgrade() -> Weight {
			T::AccountId::from(SomeType1); // Test for where clause
			T::AccountId::from(SomeType2); // Test for where clause
			Self::deposit_event(Event::Something(30));
			30
		}
		fn integrity_test() {
			T::AccountId::from(SomeType1); // Test for where clause
			T::AccountId::from(SomeType2); // Test for where clause
		}
	}

	#[noble::call]
	impl<T: Config> Noble<T>
	where T::AccountId: From<SomeType1> + From<SomeType3> + SomeAssociation1
	{
		/// Doc comment put in metadata
		#[noble::weight(Weight::from(*_foo))]
		fn foo(
			origin: OriginFor<T>,
			#[noble::compact] _foo: u32,
			_bar: u32,
		) -> DispatchResultWithPostInfo {
			T::AccountId::from(SomeType1); // Test for where clause
			T::AccountId::from(SomeType3); // Test for where clause
			let _ = origin;
			Self::deposit_event(Event::Something(3));
			Ok(().into())
		}

		/// Doc comment put in metadata
		#[noble::weight(1)]
		#[fabric_support::transactional]
		fn foo_transactional(
			_origin: OriginFor<T>,
			#[noble::compact] foo: u32,
		) -> DispatchResultWithPostInfo {
			Self::deposit_event(Event::Something(0));
			if foo == 0 {
				Err(Error::<T>::InsufficientProposersBalance)?;
			}

			Ok(().into())
		}
	}

	#[noble::error]
	pub enum Error<T> {
		/// doc comment put into metadata
		InsufficientProposersBalance,
	}

	#[noble::event]
	#[noble::metadata(BalanceOf<T> = "Balance", u32 = "Other")]
	#[noble::generate_deposit(fn deposit_event)]
	pub enum Event<T: Config> where T::AccountId: SomeAssociation1 + From<SomeType1>{
		/// doc comment put in metadata
		Proposed(<T as fabric_system::Config>::AccountId),
		/// doc
		Spending(BalanceOf<T>),
		Something(u32),
		SomethingElse(<T::AccountId as SomeAssociation1>::_1),
	}

	#[noble::storage]
	pub type ValueWhereClause<T: Config> where T::AccountId: SomeAssociation2 =
		StorageValue<_, <T::AccountId as SomeAssociation2>::_2>;

	#[noble::storage]
	pub type Value<T> = StorageValue<_, u32>;

	#[noble::type_value]
	pub fn MyDefault<T: Config>() -> u16
	where T::AccountId: From<SomeType7> + From<SomeType1> + SomeAssociation1
	{
		T::AccountId::from(SomeType7); // Test where clause works
		4u16
	}

	#[noble::storage]
	pub type Map<T: Config> where T::AccountId: From<SomeType7> =
		StorageMap<_, Blake2_128Concat, u8, u16, ValueQuery, MyDefault<T>>;

	#[noble::storage]
	pub type Map2<T> = StorageMap<_, Twox64Concat, u16, u32>;

	#[noble::storage]
	pub type DoubleMap<T> = StorageDoubleMap<_, Blake2_128Concat, u8, Twox64Concat, u16, u32>;

	#[noble::storage]
	pub type DoubleMap2<T> = StorageDoubleMap<_, Twox64Concat, u16, Blake2_128Concat, u32, u64>;

	#[noble::genesis_config]
	#[derive(Default)]
	pub struct GenesisConfig {
		_myfield: u32,
	}

	#[noble::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig
	where T::AccountId: From<SomeType1> + SomeAssociation1 + From<SomeType4>
	{
		fn build(&self) {
			T::AccountId::from(SomeType1); // Test for where clause
			T::AccountId::from(SomeType4); // Test for where clause
		}
	}

	#[noble::origin]
	#[derive(EqNoBound, RuntimeDebugNoBound, CloneNoBound, PartialEqNoBound, Encode, Decode)]
	pub struct Origin<T>(PhantomData<T>);

	#[noble::validate_unsigned]
	impl<T: Config> ValidateUnsigned for Noble<T>
	where T::AccountId: From<SomeType1> + SomeAssociation1 + From<SomeType5> + From<SomeType3>
	{
		type Call = Call<T>;
		fn validate_unsigned(
			_source: TransactionSource,
			_call: &Self::Call
		) -> TransactionValidity {
			T::AccountId::from(SomeType1); // Test for where clause
			T::AccountId::from(SomeType5); // Test for where clause
			Err(TransactionValidityError::Invalid(InvalidTransaction::Call))
		}
	}

	#[noble::inherent]
	impl<T: Config> ProvideInherent for Noble<T>
	where T::AccountId: From<SomeType1> + SomeAssociation1 + From<SomeType6> + From<SomeType3>
	{
		type Call = Call<T>;
		type Error = InherentError;

		const INHERENT_IDENTIFIER: InherentIdentifier = INHERENT_IDENTIFIER;

		fn create_inherent(_data: &InherentData) -> Option<Self::Call> {
			T::AccountId::from(SomeType1); // Test for where clause
			T::AccountId::from(SomeType6); // Test for where clause
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

// Test that a noble with non generic event and generic genesis_config is correctly handled
#[fabric_support::noble]
pub mod noble2 {
	use super::{SomeType1, SomeAssociation1};
	use fabric_support::noble_prelude::*;
	use fabric_system::noble_prelude::*;

	#[noble::config]
	pub trait Config: fabric_system::Config
	where <Self as fabric_system::Config>::AccountId: From<SomeType1> + SomeAssociation1,
	{
		type Event: From<Event> + IsType<<Self as fabric_system::Config>::Event>;
	}

	#[noble::noble]
	#[noble::generate_store(pub(crate) trait Store)]
	pub struct Noble<T>(PhantomData<T>);

	#[noble::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Noble<T>
	where T::AccountId: From<SomeType1> + SomeAssociation1,
	{
	}

	#[noble::call]
	impl<T: Config> Noble<T>
	where T::AccountId: From<SomeType1> + SomeAssociation1,
	{
	}

	#[noble::event]
	pub enum Event {
		/// Something
		Something(u32),
	}

	#[noble::genesis_config]
	pub struct GenesisConfig<T: Config>
	where T::AccountId: From<SomeType1> + SomeAssociation1,
	{
		phantom: PhantomData<T>,
	}

	impl<T: Config> Default for GenesisConfig<T>
	where T::AccountId: From<SomeType1> + SomeAssociation1,
	{
		fn default() -> Self {
			GenesisConfig {
				phantom: Default::default(),
			}
		}
	}

	#[noble::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T>
	where T::AccountId: From<SomeType1> + SomeAssociation1,
	{
		fn build(&self) {}
	}
}

fabric_support::parameter_types!(
	pub const MyGetParam: u32= 10;
	pub const MyGetParam2: u32= 11;
	pub const MyGetParam3: u32= 12;
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
	type MyGetParam = MyGetParam;
	type MyGetParam2 = MyGetParam2;
	type MyGetParam3 = MyGetParam3;
	type Balance = u64;
}

impl noble2::Config for Runtime {
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
		Example2: noble2::{Module, Call, Event, Config<T>, Storage},
	}
);

#[test]
fn transactional_works() {
	TestExternalities::default().execute_with(|| {
		fabric_system::Noble::<Runtime>::set_block_number(1);

		noble::Call::<Runtime>::foo_transactional(0).dispatch_bypass_filter(None.into())
			.err().unwrap();
		assert!(fabric_system::Noble::<Runtime>::events().is_empty());

		noble::Call::<Runtime>::foo_transactional(1).dispatch_bypass_filter(None.into()).unwrap();
		assert_eq!(
			fabric_system::Noble::<Runtime>::events().iter().map(|e| &e.event).collect::<Vec<_>>(),
			vec![&Event::noble(noble::Event::Something(0))],
		);
	})
}

#[test]
fn call_expand() {
	let call_foo = noble::Call::<Runtime>::foo(3, 0);
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
}

#[test]
fn instance_expand() {
	// Assert same type.
	let _: noble::__InherentHiddenInstance = ();
}

#[test]
fn noble_expand_deposit_event() {
	TestExternalities::default().execute_with(|| {
		fabric_system::Noble::<Runtime>::set_block_number(1);
		noble::Call::<Runtime>::foo(3, 0).dispatch_bypass_filter(None.into()).unwrap();
		assert_eq!(
			fabric_system::Noble::<Runtime>::events()[0].event,
			Event::noble(noble::Event::Something(3)),
		);
	})
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
		noble::Value::<Runtime>::put(1);
		let k = [twox_128(b"Example"), twox_128(b"Value")].concat();
		assert_eq!(unhashed::get::<u32>(&k), Some(1u32));

		noble::Map::<Runtime>::insert(1, 2);
		let mut k = [twox_128(b"Example"), twox_128(b"Map")].concat();
		k.extend(1u8.using_encoded(blake2_128_concat));
		assert_eq!(unhashed::get::<u16>(&k), Some(2u16));
		assert_eq!(&k[..32], &<noble::Map<Runtime>>::final_prefix());

		noble::Map2::<Runtime>::insert(1, 2);
		let mut k = [twox_128(b"Example"), twox_128(b"Map2")].concat();
		k.extend(1u16.using_encoded(twox_64_concat));
		assert_eq!(unhashed::get::<u32>(&k), Some(2u32));
		assert_eq!(&k[..32], &<noble::Map2<Runtime>>::final_prefix());

		noble::DoubleMap::<Runtime>::insert(&1, &2, &3);
		let mut k = [twox_128(b"Example"), twox_128(b"DoubleMap")].concat();
		k.extend(1u8.using_encoded(blake2_128_concat));
		k.extend(2u16.using_encoded(twox_64_concat));
		assert_eq!(unhashed::get::<u32>(&k), Some(3u32));
		assert_eq!(&k[..32], &<noble::DoubleMap<Runtime>>::final_prefix());

		noble::DoubleMap2::<Runtime>::insert(&1, &2, &3);
		let mut k = [twox_128(b"Example"), twox_128(b"DoubleMap2")].concat();
		k.extend(1u16.using_encoded(twox_64_concat));
		k.extend(2u32.using_encoded(blake2_128_concat));
		assert_eq!(unhashed::get::<u64>(&k), Some(3u64));
		assert_eq!(&k[..32], &<noble::DoubleMap2<Runtime>>::final_prefix());
	})
}

#[test]
fn noble_hooks_expand() {
	TestExternalities::default().execute_with(|| {
		fabric_system::Noble::<Runtime>::set_block_number(1);

		assert_eq!(AllModules::on_initialize(1), 10);
		AllModules::on_finalize(1);

		assert_eq!(noble::Noble::<Runtime>::storage_version(), None);
		assert_eq!(AllModules::on_runtime_upgrade(), 30);
		assert_eq!(
			noble::Noble::<Runtime>::storage_version(),
			Some(noble::Noble::<Runtime>::current_version()),
		);

		assert_eq!(
			fabric_system::Noble::<Runtime>::events()[0].event,
			Event::noble(noble::Event::Something(10)),
		);
		assert_eq!(
			fabric_system::Noble::<Runtime>::events()[1].event,
			Event::noble(noble::Event::Something(20)),
		);
		assert_eq!(
			fabric_system::Noble::<Runtime>::events()[2].event,
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
					name: DecodeDifferent::Decoded("ValueWhereClause".to_string()),
					modifier: StorageEntryModifier::Optional,
					ty: StorageEntryType::Plain(
						DecodeDifferent::Decoded(
							"<T::AccountId as SomeAssociation2>::_2".to_string()
						),
					),
					default: DecodeDifferent::Decoded(vec![0]),
					documentation: DecodeDifferent::Decoded(vec![]),
				},
				StorageEntryMetadata {
					name: DecodeDifferent::Decoded("Value".to_string()),
					modifier: StorageEntryModifier::Optional,
					ty: StorageEntryType::Plain(DecodeDifferent::Decoded("u32".to_string())),
					default: DecodeDifferent::Decoded(vec![0]),
					documentation: DecodeDifferent::Decoded(vec![]),
				},
				StorageEntryMetadata {
					name: DecodeDifferent::Decoded("Map".to_string()),
					modifier: StorageEntryModifier::Default,
					ty: StorageEntryType::Map {
						key: DecodeDifferent::Decoded("u8".to_string()),
						value: DecodeDifferent::Decoded("u16".to_string()),
						hasher: StorageHasher::Blake2_128Concat,
						unused: false,
					},
					default: DecodeDifferent::Decoded(vec![4, 0]),
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
					},
					FunctionArgumentMetadata {
						name: DecodeDifferent::Decoded("_bar".to_string()),
						ty: DecodeDifferent::Decoded("u32".to_string()),
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
						name: DecodeDifferent::Decoded("foo".to_string()),
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
			EventMetadata {
				name: DecodeDifferent::Decoded("SomethingElse".to_string()),
				arguments: DecodeDifferent::Decoded(vec!["<T::AccountId as SomeAssociation1>::_1".to_string()]),
				documentation: DecodeDifferent::Decoded(vec![]),
			},
		])),
		constants: DecodeDifferent::Decoded(vec![
			ModuleConstantMetadata {
				name: DecodeDifferent::Decoded("MyGetParam".to_string()),
				ty: DecodeDifferent::Decoded("u32".to_string()),
				value: DecodeDifferent::Decoded(vec![10, 0, 0, 0]),
				documentation: DecodeDifferent::Decoded(vec![
					" Some comment".to_string(),
					" Some comment".to_string(),
				]),
			},
			ModuleConstantMetadata {
				name: DecodeDifferent::Decoded("MyGetParam2".to_string()),
				ty: DecodeDifferent::Decoded("u32".to_string()),
				value: DecodeDifferent::Decoded(vec![11, 0, 0, 0]),
				documentation: DecodeDifferent::Decoded(vec![
					" Some comment".to_string(),
					" Some comment".to_string(),
				]),
			},
			ModuleConstantMetadata {
				name: DecodeDifferent::Decoded("MyGetParam3".to_string()),
				ty: DecodeDifferent::Decoded("<T::AccountId as SomeAssociation1>::_1".to_string()),
				value: DecodeDifferent::Decoded(vec![12, 0, 0, 0, 0, 0, 0, 0]),
				documentation: DecodeDifferent::Decoded(vec![]),
			},
			ModuleConstantMetadata {
				name: DecodeDifferent::Decoded("some_extra".to_string()),
				ty: DecodeDifferent::Decoded("T::AccountId".to_string()),
				value: DecodeDifferent::Decoded(vec![100, 0, 0, 0, 0, 0, 0, 0]),
				documentation: DecodeDifferent::Decoded(vec![
					" Some doc".to_string(),
					" Some doc".to_string(),
				]),
			},
			ModuleConstantMetadata {
				name: DecodeDifferent::Decoded("some_extra_extra".to_string()),
				ty: DecodeDifferent::Decoded("T::AccountId".to_string()),
				value: DecodeDifferent::Decoded(vec![0, 0, 0, 0, 0, 0, 0, 0]),
				documentation: DecodeDifferent::Decoded(vec![
					" Some doc".to_string(),
				]),
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
		RuntimeMetadata::V12(metadata) => metadata,
		_ => panic!("metadata has been bump, test needs to be updated"),
	};

	let modules_metadata = match metadata.modules {
		DecodeDifferent::Encode(modules_metadata) => modules_metadata,
		_ => unreachable!(),
	};

	let noble_metadata = ModuleMetadata::decode(&mut &modules_metadata[1].encode()[..]).unwrap();

	pretty_assertions::assert_eq!(noble_metadata, expected_noble_metadata);
}
