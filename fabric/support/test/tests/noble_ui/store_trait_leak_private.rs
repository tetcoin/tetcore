#[fabric_support::noble]
mod noble {
	use fabric_support::noble_prelude::Hooks;
	use fabric_system::noble_prelude::BlockNumberFor;
	use fabric_support::noble_prelude::StorageValue;

	#[noble::config]
	pub trait Config: fabric_system::Config {}

	#[noble::noble]
	#[noble::generate_store(pub trait Store)]
	pub struct Noble<T>(core::marker::PhantomData<T>);

	#[noble::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Noble<T> {}

	#[noble::call]
	impl<T: Config> Noble<T> {}

	#[noble::storage]
	type Foo<T> = StorageValue<_, u8>;
}

fn main() {
}
