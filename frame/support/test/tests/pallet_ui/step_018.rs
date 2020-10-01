#[frame_support::pallet]
mod pallet {
	use frame_support::pallet_prelude::*;

	#[pallet::config]
	pub trait Trait: frame_system::Trait {}

	#[pallet::module]
	pub struct Module<T> {}

	#[pallet::module_interface]
	impl<T: Trait> ModuleInterface for Module<T> {}

	#[pallet::call]
	impl Foo {}
}

fn main() {
}
