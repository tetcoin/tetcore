#[frame_support::pallet]
mod pallet {
	#[pallet::config]
	pub trait Trait: frame_system::Trait {}

	#[pallet::module]
	pub enum Foo {}
}

fn main() {
}
