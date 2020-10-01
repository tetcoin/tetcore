#[frame_support::pallet]
mod pallet {
	use frame_support::pallet_prelude::Interface;
	use frame_system::pallet_prelude::BlockNumberFor;

	#[pallet::config]
	pub trait Config: frame_system::Config {}

	#[pallet::pallet]
	pub struct Pallet<T, I = ()>(core::marker::PhantomData<(T, I)>);

	#[pallet::interface]
	impl<T: Config<I>, I: 'static> Interface<BlockNumberFor<T>> for Pallet<T, I> {}

	#[pallet::call]
	impl<T: Config<I>, I: 'static> Pallet<T, I> {}
}

fn main() {
}
