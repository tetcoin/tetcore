#[fabric_support::noble]
mod noble {
	use fabric_support::noble_prelude::{Hooks, PhantomData};

	#[noble::config]
	pub trait Config: fabric_system::Config {}

	#[noble::noble]
	pub struct Noble<T>(PhantomData<T>);

	#[noble::hooks]
	impl<T: Config> Hooks for Noble<T> {}

	#[noble::call]
	impl<T: Config> Noble<T> {}
}

fn main() {
}
