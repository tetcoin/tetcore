#[frame_support::pallet]
mod pallet {
	use frame_support::pallet_prelude::{ModuleInterface, ProvideInherent};
	use frame_system::pallet_prelude::BlockNumberFor;

	#[pallet::config]
	pub trait Trait: frame_system::Trait {}

	#[pallet::module]
	pub struct Module<T>(core::marker::PhantomData<T>);

	#[pallet::module_interface]
	impl<T: Trait> ModuleInterface<BlockNumberFor<T>> for Module<T> {}

	#[pallet::call]
	impl<T: Trait> Module<T> {}

	#[pallet::inherent]
	impl<T: Trait> ProvideInherent for Module<T> {}
}

fn main() {
}
