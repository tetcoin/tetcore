#[frame_support::pallet]
mod pallet {
	use frame_support::pallet_prelude::ModuleInterface;
	use frame_system::pallet_prelude::BlockNumberFor;

	#[pallet::config]
	pub trait Trait: frame_system::Trait {
		type Bar;
		type Event;
	}

	#[pallet::module]
	pub struct Module<T>(core::marker::PhantomData<T>);

	#[pallet::module_interface]
	impl<T: Trait> ModuleInterface<BlockNumberFor<T>> for Module<T> {}

	#[pallet::call]
	impl<T: Trait> Module<T> {}

	#[pallet::event]
	pub enum Event<T: Trait> {
		B { b: T::Bar },
	}
}

fn main() {
}
