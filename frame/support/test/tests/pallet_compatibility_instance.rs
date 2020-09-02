use sp_runtime::traits::Block as _;

mod pallet_old {
	use frame_support::{
		decl_storage, decl_error, decl_event, decl_module, weights::Weight, traits::Get, Parameter
	};
	use frame_system::ensure_root;

	pub trait Trait<I: Instance = DefaultInstance>: frame_system::Trait {
		type SomeConst: Get<Self::Balance>;
		type Balance: Parameter + codec::HasCompact + From<u32> + Into<Weight> + Default;
		type Event: From<Event<Self, I>> + Into<<Self as frame_system::Trait>::Event>;
	}

	decl_storage! {
		trait Store for Module<T: Trait<I>, I: Instance = DefaultInstance> as Example {
			/// Some documentation
			Dummy get(fn dummy) config(): Option<T::Balance>;
			Bar get(fn bar) config(): map hasher(blake2_128_concat) T::AccountId => T::Balance;
			Foo get(fn foo) config(): T::Balance = 3.into();
			Double get(fn double): double_map hasher(blake2_128_concat) u32, hasher(twox_64_concat) u64 => u16;
		}
	}

	decl_event!(
		pub enum Event<T, I = DefaultInstance> where Balance = <T as Trait<I>>::Balance {
			/// Dummy event, just here so there's a generic type that's used.
			Dummy(Balance),
		}
	);

	decl_module! {
		pub struct Module<T: Trait<I>, I: Instance = DefaultInstance> for enum Call where origin: T::Origin {
			type Error = Error<T, I>;
			fn deposit_event() = default;
			const SomeConst: T::Balance = T::SomeConst::get();

			#[weight = <T::Balance as Into<Weight>>::into(new_value.clone())]
			fn set_dummy(origin, #[compact] new_value: T::Balance) {
				ensure_root(origin)?;

				<Dummy<T, I>>::put(&new_value);
				Self::deposit_event(RawEvent::Dummy(new_value));
			}

			fn on_initialize(_n: T::BlockNumber) -> Weight {
				<Dummy<T, I>>::put(T::Balance::from(10));
				10
			}

			fn on_finalize(_n: T::BlockNumber) {
				<Dummy<T, I>>::put(T::Balance::from(11));
			}
		}
	}

	decl_error! {
		pub enum Error for Module<T: Trait<I>, I: Instance> {
			/// Some wrong behavior
			Wrong,
		}
	}
}

#[frame_support::pallet(Example)]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use frame_system::ensure_root;

	#[pallet::trait_]
	pub trait Trait<I: Instance = DefaultInstance>: frame_system::Trait {
		type Balance: Parameter + codec::HasCompact + From<u32> + Into<Weight> + Default
			+ MaybeSerializeDeserialize;
		#[pallet::const_]
		type SomeConst: Get<Self::Balance>;
		type Event: From<Event<Self, I>> + IsType<<Self as frame_system::Trait>::Event>;
	}

	#[pallet::module]
	#[pallet::generate(fn deposit_event)]
	pub struct Module<T, I = DefaultInstance>(PhantomData<(T, I)>);

	#[pallet::module_interface]
	impl<T: Trait<I>, I: Instance> ModuleInterface<T::BlockNumber> for Module<T, I> {
		fn on_initialize(_n: T::BlockNumber) -> Weight {
			<Dummy<T, I>>::put(T::Balance::from(10));
			10
		}

		fn on_finalize(_n: T::BlockNumber) {
			<Dummy<T, I>>::put(T::Balance::from(11));
		}
	}

	#[pallet::call]
	impl<T: Trait<I>, I: Instance> Call for Module<T, I> {
		#[pallet::weight(<T::Balance as Into<Weight>>::into(new_value.clone()))]
		fn set_dummy(origin: OriginFor<T>, #[pallet::compact] new_value: T::Balance) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;

			<Dummy<T, I>>::put(&new_value);
			Self::deposit_event(Event::Dummy(new_value));

			Ok(().into())
		}
	}

	#[pallet::error]
	pub enum Error<T, I = DefaultInstance> {
		/// Some wrong behavior
		Wrong,
	}

	#[pallet::event]
	pub enum Event<T: Trait<I>, I: Instance = DefaultInstance> {
		/// Dummy event, just here so there's a generic type that's used.
		Dummy(T::Balance),
	}

	#[pallet::storage]
	/// Some documentation
	type Dummy<T: Trait<I>, I: Instance = DefaultInstance> = StorageValueType<_, T::Balance, OptionQuery>;

	#[pallet::storage]
	type Bar<T: Trait<I>, I: Instance = DefaultInstance> =
		StorageMapType<_, Blake2_128Concat, T::AccountId, T::Balance, ValueQuery>;

	#[pallet::storage]
	type Foo<T: Trait<I>, I: Instance = DefaultInstance> =
		StorageValueType<_, T::Balance, ValueQuery, OnFooEmpty<T, I>>;
	pub struct OnFooEmpty<T: Trait<I>, I: Instance>(PhantomData<(T, I)>);
	impl<T: Trait<I>, I: Instance> Get<T::Balance> for OnFooEmpty<T, I> { fn get() -> T::Balance { 3.into() } }

	#[pallet::storage]
	type Double<I: Instance = DefaultInstance> = StorageDoubleMapType<
		_, Blake2_128Concat, u32, Twox64Concat, u64, u16, ValueQuery
	>;

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Trait<I>, I: Instance = DefaultInstance> {
		dummy: Option<T::Balance>,
		bar: Vec<(T::AccountId, T::Balance)>,
		foo: T::Balance,
	}

	impl<T: Trait<I>, I: Instance> Default for GenesisConfig<T, I> {
		fn default() -> Self {
			GenesisConfig {
				dummy: Default::default(),
				bar: Default::default(),
				foo: OnFooEmpty::<T, I>::get(),
			}
		}
	}

	#[pallet::genesis_build]
	impl<T: Trait<I>, I: Instance> GenesisBuilder<T, I> for GenesisConfig<T, I> {
		fn build(&self) {
			if let Some(dummy) = self.dummy.as_ref() {
				<Dummy<T, I>>::put(dummy);
			}
			for (k, v) in &self.bar {
				<Bar<T, I>>::insert(k, v);
			}
			<Foo<T, I>>::put(&self.foo);
		}
	}
}

frame_support::parameter_types!(
	pub const SomeConst: u64 = 10;
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
	type ModuleToIndex = ();
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
}
impl pallet::Trait for Runtime {
	type Event = Event;
	type SomeConst = SomeConst;
	type Balance = u64;
}
impl pallet::Trait<pallet::Instance2> for Runtime {
	type Event = Event;
	type SomeConst = SomeConst;
	type Balance = u64;
}
impl pallet::Trait<pallet::Instance3> for Runtime {
	type Event = Event;
	type SomeConst = SomeConst;
	type Balance = u64;
}
impl pallet_old::Trait for Runtime {
	type Event = Event;
	type SomeConst = SomeConst;
	type Balance = u64;
}
impl pallet_old::Trait<pallet_old::Instance2> for Runtime {
	type Event = Event;
	type SomeConst = SomeConst;
	type Balance = u64;
}
impl pallet_old::Trait<pallet_old::Instance3> for Runtime {
	type Event = Event;
	type SomeConst = SomeConst;
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
		Pallet: pallet::{Module, Call, Event<T>, Config<T>, Storage},
		PalletOld: pallet_old::{Module, Call, Event<T>, Config<T>, Storage},
		Pallet2: pallet::<Instance2>::{Module, Call, Event<T>, Config<T>, Storage},
		PalletOld2: pallet_old::<Instance2>::{Module, Call, Event<T>, Config<T>, Storage},
		Pallet3: pallet::<Instance3>::{Module, Call, Event<T>, Config<T>, Storage},
		PalletOld3: pallet_old::<Instance3>::{Module, Call, Event<T>, Config<T>, Storage},
	}
);

#[cfg(test)]
mod test {
	use super::Runtime;
	use super::pallet;
	use super::pallet_old;
	use codec::{Decode, Encode};
	use sp_runtime::BuildStorage as _;

	#[test]
	fn metadata() {
		let metadata = Runtime::metadata();
		let modules = match metadata.1 {
			frame_metadata::RuntimeMetadata::V11(frame_metadata::RuntimeMetadataV11 {
				modules: frame_metadata::DecodeDifferent::Encode(m),
				..
			}) => m,
			_ => unreachable!(),
		};
		for i in vec![1, 3, 5].into_iter() {
			pretty_assertions::assert_eq!(modules[i].storage, modules[i+1].storage);
			pretty_assertions::assert_eq!(modules[i].calls, modules[i+1].calls);
			pretty_assertions::assert_eq!(modules[i].event, modules[i+1].event);
			pretty_assertions::assert_eq!(modules[i].constants, modules[i+1].constants);
			pretty_assertions::assert_eq!(modules[i].errors, modules[i+1].errors);
		}
	}

	#[test]
	fn types() {
		assert_eq!(
			pallet_old::Event::<Runtime>::decode(&mut &pallet::Event::<Runtime>::Dummy(10).encode()[..]).unwrap(),
			pallet_old::Event::<Runtime>::Dummy(10),
		);

		assert_eq!(
			pallet_old::Call::<Runtime>::decode(&mut &pallet::Call::<Runtime>::set_dummy(10).encode()[..]).unwrap(),
			pallet_old::Call::<Runtime>::set_dummy(10),
		);
	}

	#[test]
	fn execution() {
		let storage = super::GenesisConfig {
			pallet: Default::default(),
			pallet_Instance2: Default::default(),
			pallet_Instance3: Default::default(),
			pallet_old: Default::default(),
			pallet_old_Instance2: Default::default(),
			pallet_old_Instance3: Default::default(),
		}.build_storage().unwrap();

		// storage.execute_with(|| {
		// 	// TODO TODO: some storage tests
		// })
	}
}
