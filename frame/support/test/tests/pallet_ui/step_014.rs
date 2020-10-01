#[frame_support::pallet]
mod pallet {
	#[pallet::config]
	pub trait Config: frame_system::Config {}

	#[pallet::pallet]
	pub struct Pallet<T> {}

	#[pallet::interface]
	impl Foo {}
}

fn main() {
}
