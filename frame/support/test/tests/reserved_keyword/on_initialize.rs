macro_rules! reserved {
	($($reserved:ident)*) => {
		$(
			mod $reserved {
				pub use frame_support::dispatch;

				/// Temporary keep old name Trait, to be removed alongside old macro.
				pub trait Trait: Config {}
				impl<Runtime: Config> Trait for Runtime {}
				/// Temporary keep old module name, to be removed alongside old macro.
#[allow(unused)]
				pub type Pallet<T> = Module<T>;

				pub trait Config {
					type Origin;
					type BlockNumber: Into<u32>;
				}

				pub mod system {
					use frame_support::dispatch;

					pub fn ensure_root<R>(_: R) -> dispatch::DispatchResult {
						Ok(())
					}
				}

				frame_support::decl_module! {
					pub struct Module<T: Trait> for enum Call where origin: T::Origin, system=self {
						#[weight = 0]
						fn $reserved(_origin) -> dispatch::DispatchResult { unreachable!() }
					}
				}
			}
		)*
	}
}

reserved!(on_finalize on_initialize on_runtime_upgrade offchain_worker deposit_event);

fn main() {}
