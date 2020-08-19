#[frame_support::pallet(Example)]
mod pallet {
	#[pallet::trait_]
	pub trait Trait: frame_system::Trait {}

	#[pallet::module]
	pub struct Module<T> {}

	#[pallet::module_interface]
	pub enum Foo {}
}

fn main() {
}
