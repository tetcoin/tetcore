#[fabric_support::noble]
mod noble {
	use fabric_support::noble_prelude::{Hooks, DispatchResultWithPostInfo};
	use fabric_system::noble_prelude::{BlockNumberFor, OriginFor};

	#[noble::config]
	pub trait Config: fabric_system::Config {
		type Bar;
	}

	#[noble::noble]
	pub struct Noble<T>(core::marker::PhantomData<T>);

	#[noble::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Noble<T> {}

	#[noble::call]
	impl<T: Config> Noble<T> {
		#[noble::weight(0)]
		fn foo(origin: OriginFor<T>, bar: T::Bar) -> DispatchResultWithPostInfo {
			Ok(().into())
		}
	}
}

fn main() {
}
