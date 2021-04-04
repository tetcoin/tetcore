#[fabric_support::noble]
mod noble {
	use fabric_support::noble_prelude::Hooks;
	use fabric_system::noble_prelude::BlockNumberFor;

	#[noble::config]
	pub trait Config: fabric_system::Config {}

	#[noble::noble]
	pub struct Noble<T, I = ()>(core::marker::PhantomData<(T, I)>);

	#[noble::hooks]
	impl<T: Config<I>, I: 'static> Hooks<BlockNumberFor<T>> for Noble<T, I> {}

	#[noble::call]
	impl<T: Config<I>, I: 'static> Noble<T, I> {}
}

fn main() {
}
