#[frame_support::pallet(Example)]
mod pallet {
	use frame_support::pallet_prelude::ModuleInterface;
	use frame_system::pallet_prelude::BlockNumberFor;

	#[pallet::trait_]
	pub trait Trait: frame_system::Trait {}

	#[pallet::module]
	pub struct Module<T>(core::marker::PhantomData<T>);

	#[pallet::module_interface]
	impl<T: Trait> ModuleInterface<BlockNumberFor<T>> for Module<T> {}

	#[pallet::call]
	impl<T: Trait> Module<T> {}

	#[pallet::genesis_build]
	impl Foo {}
}

fn main() {
}
