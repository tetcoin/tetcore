#[frame_support::pallet(Example)]
mod pallet {
	use frame_support::pallet_prelude::ModuleInterface;
	use frame_system::pallet_prelude::BlockNumberFor;

	#[pallet::trait_]
	pub trait Trait: frame_system::Trait {}

	#[pallet::module]
	pub struct Module<T, I = DefaultInstance>(core::marker::PhantomData<(T, I)>);

	#[pallet::module_interface]
	impl<T: Trait<I>, I: Instance> ModuleInterface<BlockNumberFor<T>> for Module<T, I> {}

	#[pallet::call]
	impl<T: Trait<I>, I: Instance> Module<T, I> {}
}

fn main() {
}
