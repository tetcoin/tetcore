#[frame_support::pallet]
mod pallet {
	mod balance {
		pub trait Config: frame_system::Config {}
	}
	mod timestamp {
		pub trait Config: frame_system::Config {}
	}

	#[pallet::config]
	pub trait Config: balance::Config + timestamp::Config {}
}

fn main() {
}
