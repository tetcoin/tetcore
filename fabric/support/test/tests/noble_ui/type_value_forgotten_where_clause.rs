#[fabric_support::noble]
mod noble {
	use fabric_support::noble_prelude::{Hooks, PhantomData};
	use fabric_system::noble_prelude::BlockNumberFor;

	#[noble::config]
	pub trait Config: fabric_system::Config
	where <Self as fabric_system::Config>::AccountId: From<u32>
	{}

	#[noble::noble]
	pub struct Noble<T>(PhantomData<T>);

	#[noble::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Noble<T>
	where <T as fabric_system::Config>::AccountId: From<u32>
	{}

	#[noble::call]
	impl<T: Config> Noble<T>
	where <T as fabric_system::Config>::AccountId: From<u32>
	{}

	#[noble::type_value] fn Foo<T: Config>() -> u32 { 3u32 }
}

fn main() {
}
