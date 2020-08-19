#[frame_support::pallet(Example)]
mod pallet {
	use frame_support::pallet_prelude::*;
	#[pallet::trait_]
	pub trait Trait: frame_system::Trait {}

	#[pallet::module]
	pub struct Module<T> {}

	#[pallet::module_interface]
	impl<T> path::ModuleInterface for Module<T> {}
}

fn main() {
}
