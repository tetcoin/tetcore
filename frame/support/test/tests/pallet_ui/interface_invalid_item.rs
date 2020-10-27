#[frame_support::pallet]
mod pallet {
	use frame_support::pallet_prelude::{Interface, PhantomData};

	#[pallet::config]
	pub trait Config: frame_system::Config {}

	#[pallet::pallet]
	pub struct Pallet<T>(PhantomData<T>);

	#[pallet::interface]
	impl<T: Config> Interface for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {}
}

fn main() {
}
