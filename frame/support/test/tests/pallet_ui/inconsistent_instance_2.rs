#[frame_support::pallet]
mod pallet {
	use frame_support::pallet_prelude::ModuleInterface;
	use frame_system::pallet_prelude::BlockNumberFor;

	#[pallet::trait_]
	pub trait Trait: frame_system::Trait {}

	#[pallet::module]
	pub struct Module<T, I = ()>(core::marker::PhantomData<(T, I)>);

	#[pallet::module_interface]
	impl<T: Trait<I>, I: 'static> ModuleInterface<BlockNumberFor<T>> for Module<T, I> {}

	#[pallet::call]
	impl<T: Trait<I>, I: 'static> Module<T, I> {}
}

fn main() {
}
