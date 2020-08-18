#[frame_support::pallet(Example)]
mod pallet {
	use frame_support::pallet_prelude::ModuleInterface;
	use frame_system::pallet_prelude::BlockNumberFor;

	#[pallet::trait_]
	pub trait Trait {}

	#[pallet::module]
	pub struct Module<T> {}
	// TODO TODO: issue: error is very difficult to understand that it is because Trait bound
	// frame_system::Trait

	#[pallet::module_interface]
	impl<T: Trait> ModuleInterface<BlockNumberFor<T>> for Module<T> {}

	#[pallet::call]
	impl<T: Trait> Call for Module<T> {}
}

fn main() {
}
