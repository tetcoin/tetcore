#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FABRIC and the core library of Tetcore FABRIC nobles:
/// https://tetcoin.org/docs/en/knowledgebase/runtime/fabric

pub use noble::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[fabric_support::noble]
pub mod noble {
	use fabric_support::{dispatch::DispatchResultWithPostInfo, noble_prelude::*};
	use fabric_system::noble_prelude::*;

	/// Configure the noble by specifying the parameters and types on which it depends.
	#[noble::config]
	pub trait Config: fabric_system::Config {
		/// Because this noble emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as fabric_system::Config>::Event>;
	}

	#[noble::noble]
	#[noble::generate_store(pub(super) trait Store)]
	pub struct Noble<T>(PhantomData<T>);

	// The noble's runtime storage items.
	// https://tetcoin.org/docs/en/knowledgebase/runtime/storage
	#[noble::storage]
	#[noble::getter(fn something)]
	// Learn more about declaring storage items:
	// https://tetcoin.org/docs/en/knowledgebase/runtime/storage#declaring-storage-items
	pub type Something<T> = StorageValue<_, u32>;

	// Nobles use events to inform users when important changes are made.
	// https://tetcoin.org/docs/en/knowledgebase/runtime/events
	#[noble::event]
	#[noble::metadata(T::AccountId = "AccountId")]
	#[noble::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored(u32, T::AccountId),
	}

	// Errors inform users that something went wrong.
	#[noble::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}

	#[noble::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Noble<T> {}

	// Dispatchable functions allows users to interact with the noble and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[noble::call]
	impl<T:Config> Noble<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[noble::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResultWithPostInfo {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://tetcoin.org/docs/en/knowledgebase/runtime/origin
			let who = ensure_signed(origin)?;

			// Update storage.
			<Something<T>>::put(something);

			// Emit an event.
			Self::deposit_event(Event::SomethingStored(something, who));
			// Return a successful DispatchResultWithPostInfo
			Ok(().into())
		}

		/// An example dispatchable that may throw a custom error.
		#[noble::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn cause_error(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			let _who = ensure_signed(origin)?;

			// Read a value from storage.
			match <Something<T>>::get() {
				// Return an error if the value has not been set.
				None => Err(Error::<T>::NoneValue)?,
				Some(old) => {
					// Increment the value read from storage; will error in the event of overflow.
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					// Update the value in storage with the incremented result.
					<Something<T>>::put(new);
					Ok(().into())
				},
			}
		}
	}
}
