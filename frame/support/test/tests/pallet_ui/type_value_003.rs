#[frame_support::pallet]
mod pallet {
	use frame_support::pallet_prelude::{Interface, PhantomData};
	use frame_system::pallet_prelude::BlockNumberFor;

	#[pallet::config]
	pub trait Config: frame_system::Config {}

	#[pallet::pallet]
	pub struct Pallet<T>(PhantomData<T>);

	#[pallet::interface]
	impl<T: Config> Interface<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {}

	#[pallet::type_value] fn Foo() -> u32 {
		// Just wrong code to see span
		u32::new()
	}
}

fn main() {
}
