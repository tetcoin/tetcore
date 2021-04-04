#[fabric_support::noble]
mod noble {
	use fabric_support::noble_prelude::{Hooks, PhantomData};
	use fabric_system::noble_prelude::BlockNumberFor;

	#[noble::config]
	pub trait Config: fabric_system::Config {}

	#[noble::noble]
	pub struct Noble<T>(PhantomData<T>);

	#[noble::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Noble<T> {}

	#[noble::call]
	impl<T: Config> Noble<T> {}

	#[noble::type_value] fn Foo() -> u32 {
		// Just wrong code to see span
		u32::new()
	}
}

fn main() {
}
