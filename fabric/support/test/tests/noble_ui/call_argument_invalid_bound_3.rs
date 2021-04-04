#[fabric_support::noble]
mod noble {
	use fabric_support::noble_prelude::{Hooks, DispatchResultWithPostInfo};
	use fabric_system::noble_prelude::{BlockNumberFor, OriginFor};
	use codec::{Encode, Decode};

	#[noble::config]
	pub trait Config: fabric_system::Config {}

	#[noble::noble]
	pub struct Noble<T>(core::marker::PhantomData<T>);

	#[noble::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Noble<T> {}

	#[derive(Encode, Decode)]
	struct Bar;

	#[noble::call]
	impl<T: Config> Noble<T> {
		#[noble::weight(0)]
		fn foo(origin: OriginFor<T>, bar: Bar) -> DispatchResultWithPostInfo {
			Ok(().into())
		}
	}
}

fn main() {
}
