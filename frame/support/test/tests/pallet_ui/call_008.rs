#[frame_support::pallet(Example)]
mod pallet {
	use frame_support::pallet_prelude::{ModuleInterface, DispatchResultWithPostInfo};
	use frame_system::pallet_prelude::{BlockNumberFor, OriginFor};

	#[pallet::trait_]
	pub trait Trait: frame_system::Trait {
		type Bar;
	}

	#[pallet::module]
	pub struct Module<T>(core::marker::PhantomData<T>);

	#[pallet::module_interface]
	impl<T: Trait> ModuleInterface<BlockNumberFor<T>> for Module<T> {}

	#[pallet::call]
	impl<T: Trait> Module<T> {
		#[pallet::weight(0)]
		fn foo(origin: OriginFor<T>, bar: T::Bar) -> DispatchResultWithPostInfo {
			Ok(().into())
		}
	}
}

fn main() {
}
