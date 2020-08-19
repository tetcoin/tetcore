#[frame_support::pallet(Example)]
mod pallet {
	mod balance {
		pub trait Trait: frame_system::Trait {}
	}
	mod timestamp {
		pub trait Trait: frame_system::Trait {}
	}

	#[pallet::trait_]
	pub trait Trait: balance::Trait + timestamp::Trait {}
}

fn main() {
}
