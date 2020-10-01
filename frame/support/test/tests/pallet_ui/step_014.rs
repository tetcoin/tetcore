#[frame_support::pallet]
mod pallet {
	#[pallet::trait_]
	pub trait Trait: frame_system::Trait {}

	#[pallet::module]
	pub struct Module<T> {}

	#[pallet::module_interface]
	impl Foo {}
}

fn main() {
}
