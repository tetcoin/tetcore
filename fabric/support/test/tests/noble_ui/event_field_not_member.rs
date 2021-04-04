#[fabric_support::noble]
mod noble {
	use fabric_support::noble_prelude::{Hooks, IsType};
	use fabric_system::noble_prelude::BlockNumberFor;

	#[noble::config]
	pub trait Config: fabric_system::Config {
		type Bar;
		type Event: IsType<<Self as fabric_system::Config>::Event> + From<Event<Self>>;
	}

	#[noble::noble]
	pub struct Noble<T>(core::marker::PhantomData<T>);

	#[noble::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Noble<T> {}

	#[noble::call]
	impl<T: Config> Noble<T> {}

	#[noble::event]
	pub enum Event<T: Config> {
		B { b: T::Bar },
	}
}

fn main() {
}
