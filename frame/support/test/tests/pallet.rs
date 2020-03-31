#[frame_support::pallet(Example)]
// NOTE: Example is name of the pallet, it will be used as unique identifier for storage
pub mod pallet {
	use frame_support::pallet_prelude::*; // Import various types used in pallet definition
	use frame_system::pallet_prelude::*; // OriginFor helper type for implementing dispatchables.

	type BalanceOf<T> = <T as Trait>::Balance;

	// Define the generic parameter of the pallet
	// The macro checks trait generics: is expected none or `I: Instance = DefaultInstance`.
	// The macro parses `#[pallet::const_]` attributes: used to generate constant metadata,
	// expected syntax is `type $IDENT: Get<$TYPE>;`.
	#[pallet::trait_]
	pub trait Trait: frame_system::Trait {
		#[pallet::const_] // put the constant in metadata
		type MyGetParam: Get<u32>;
		type Balance: Parameter + Default;
	}

	// Define the module struct placeholder, various pallet function are implemented on it.
	// The macro checks struct generics: is expected `T` or `T, I = DefaultInstance`
	#[pallet::module]
	pub struct Module<T>(PhantomData<T>);

	// Implement on the module interface on module.
	// The macro checks:
	// * trait is `ModuleInterface` (imported from pallet_prelude)
	// * struct is `Module<T>` or `Module<T, I>`
	#[pallet::module_interface]
	impl<T: Trait> ModuleInterface<BlockNumberFor<T>> for Module<T> {
	}

	// Declare Call struct and implement dispatchables.
	//
	// WARNING: Each parameter used in functions must implement: Clone, Debug, Eq, PartialEq,
	// Codec.
	//
	// The macro checks:
	// * module is `Module<T>` or `Module<T, I>`
	// * trait is `Call`
	// * each dispatchable functions first argument is `origin: OriginFor<T>` (OriginFor is
	//   imported from frame_system.
	//
	// The macro parse `#[pallet::compact]` attributes, function parameter with this attribute
	// will be encoded/decoded using compact codec in implementation of codec for the enum
	// `Call`.
	//
	// The macro generate the enum `Call` with a variant for each dispatchable and implements
	// codec, Eq, PartialEq, Clone and Debug.
	#[pallet::call]
	impl<T: Trait> Call for Module<T> {
		/// Doc comment put in metadata
		#[pallet::weight = 0] // Defines weight for call (function parameters are in scope)
		fn toto(origin: OriginFor<T>, #[pallet::compact] _foo: u32) -> DispatchResultWithPostInfo {
			let _ = origin;
			unimplemented!();
		}
	}

	// Declare pallet Error enum. (this is optional)
	// The macro checks enum generics and that each variant is unit.
	// The macro generate error metadata using doc comment on each variant.
	#[pallet::error]
	pub enum Error<T> {
		/// doc comment put into metadata
		InsufficientProposersBalance,
	}

	// Declare pallet Event enum. (this is optional)
	//
	// WARNING: Each type used in variants must implement: Clone, Debug, Eq, PartialEq, Codec.
	//
	// The macro generates event metadata, and derive Clone, Debug, Eq, PartialEq and Codec
	#[pallet::event]
	// Additional argument to specify the metadata to use for given type.
	#[pallet::metadata(BalanceOf<T> = Balance, u32 = Other)]
	pub enum Event<T: Trait> {
		/// doc comment put in metadata
		// `<T as frame_system::Trait>::AccountId` is not defined in metadata list, the last
		// segment is put into metadata, i.e. `AccountId`
		Proposed(<T as frame_system::Trait>::AccountId),
		/// doc
		// here metadata will be `Balance` as define in metadata list
		Spending(BalanceOf<T>),
		// here metadata will be `Other` as define in metadata list
		Something(u32),
	}

	// Declare a storage, any amount of storage can be declared.
	//
	// Is expected either `StorageValueType`, `StorageMapType` or `StorageDoubleMapType`.
	// The macro generates for struct `$identP` (for storage of name `$ident`) and implement
	// storage instance on it.
	// The macro macro expand the metadata for the storage with the type used:
	// * For storage value the type for value will be copied into metadata
	// * For storage map the type for value and the type for key will be copied into metadata
	// * For storage double map the type for value, key1, and key2 will be copied into
	//   metadata.
	//
	// NOTE: for storage hasher, the type is not copied because storage hasher trait already
	// implements metadata. Thus generic storage hasher is supported.
	#[pallet::storage] #[allow(type_alias_bounds)]
	type MyStorageValue<T: Trait> = StorageValueType<MyStorageValueP, T::Balance, ValueQuery>;

	// Another declaration
	#[pallet::storage]
	type MyStorage = StorageMapType<MyStorageP, Blake2_128Concat, u32, u32>;

	// Declare genesis config. (This is optional)
	//
	// The macro accept either type alias or struct or enum, it checks generics are consistent.
	//
	// Type must implement `Default` traits
	#[pallet::genesis_config]
	#[derive(Default)]
	pub struct GenesisConfig {
		_myfield: u32,
	}

	// Declare genesis builder. (This is need only if GenesisConfig is declared)
	#[pallet::genesis_build]
	impl<T: Trait> GenesisBuilder<T> for GenesisConfig {
		fn build(&self) {}
	}

	// Declare a pallet origin. (this is optional)
	//
	// The macro accept type alias or struct or enum, it checks generics are consistent.
	#[pallet::origin]
	pub struct Origin<T>(PhantomData<T>);

	// Declare inherent provider for module. (this is optional)
	//
	// The macro checks module is `Module<T>` or `Module<T, I>` and trait is `ProvideInherent`
	#[pallet::inherent]
	impl<T: Trait> ProvideInherent for Module<T> {
		type Call = Call<T>;
		type Error = InherentError;

		const INHERENT_IDENTIFIER: InherentIdentifier = INHERENT_IDENTIFIER;

		fn create_inherent(_data: &InherentData) -> Option<Self::Call> {
			unimplemented!();
		}
	}

	// Regular rust code needed for implementing ProvideInherent trait

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
