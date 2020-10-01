#[frame_support::pallet]
mod pallet {
	use frame_support::pallet_prelude::*;
	#[pallet::config]
	pub trait Config: frame_system::Config {}

	#[pallet::pallet]
	pub struct Pallet<T> {}

	#[pallet::interface]
	impl<T> path::Interface for Pallet<T> {}
}

fn main() {
}
