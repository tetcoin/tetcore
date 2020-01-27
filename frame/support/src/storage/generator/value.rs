// Copyright 2019-2020 Parity Technologies (UK) Ltd.
// This file is part of Substrate.

// Substrate is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Substrate is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Substrate.  If not, see <http://www.gnu.org/licenses/>.

#[cfg(not(feature = "std"))]
use sp_std::prelude::*;
use codec::{FullCodec, Encode, EncodeAppend, EncodeLike, Decode};
use crate::{storage::{self, storage_space::StorageSpace}, traits::Len};

/// Generator for `StorageValue` used by `decl_storage`.
///
/// By default value is stored at:
/// ```nocompile
/// Twox128(module_prefix) ++ Twox128(storage_prefix)
/// ```
pub trait StorageValue<T: FullCodec> {
	/// The type that get/take returns.
	type Query;

	/// The space used in the trie to store its information. This space must not collide with any
	/// other storage space.
	const STORAGE_SPACE: Self::StorageSpace;

	/// The type of the storage space used.
	type StorageSpace: StorageSpace;

	/// Convert an optional value retrieved from storage to the type queried.
	fn from_optional_value_to_query(v: Option<T>) -> Self::Query;

	/// Convert a query to an optional value into storage.
	fn from_query_to_optional_value(v: Self::Query) -> Option<T>;
}

impl<T: FullCodec, G: StorageValue<T>> storage::StorageValue<T> for G {
	type Query = G::Query;

	fn exists() -> bool {
		Self::STORAGE_SPACE.exists(&[0; 0])
	}

	fn get() -> Self::Query {
		G::from_optional_value_to_query(Self::STORAGE_SPACE.get_decode_and_warn(&[0; 0]))
	}

	fn translate<O: Decode, F: FnOnce(Option<O>) -> Option<T>>(f: F) -> Result<Option<T>, ()> {
		// attempt to get the length directly.
		let maybe_old = match Self::STORAGE_SPACE.get(&[0; 0]) {
			Some(old_data) => Some(O::decode(&mut &old_data[..]).map_err(|_| ())?),
			None => None,
		};
		let maybe_new = f(maybe_old);
		if let Some(new) = maybe_new.as_ref() {
			G::put(new)
		} else {
			G::kill()
		}
		Ok(maybe_new)
	}

	fn put<Arg: EncodeLike<T>>(val: Arg) {
		val.using_encoded(|val| Self::STORAGE_SPACE.put(&[0; 0], &val))
	}

	fn kill() {
		Self::STORAGE_SPACE.kill(&[0; 0])
	}

	fn mutate<R, F: FnOnce(&mut G::Query) -> R>(f: F) -> R {
		let mut val = G::get();

		let ret = f(&mut val);
		match G::from_query_to_optional_value(val) {
			Some(ref val) => G::put(val),
			None => G::kill(),
		}
		ret
	}

	fn take() -> G::Query {
		let value = Self::STORAGE_SPACE.get_decode_and_warn(&[0; 0]);
		if value.is_some() {
			G::kill()
		}
		G::from_optional_value_to_query(value)
	}

	/// Append the given items to the value in the storage.
	///
	/// `T` is required to implement `codec::EncodeAppend`.
	fn append<Items, Item, EncodeLikeItem>(items: Items) -> Result<(), &'static str>
	where
		Item: Encode,
		EncodeLikeItem: EncodeLike<Item>,
		T: EncodeAppend<Item=Item>,
		Items: IntoIterator<Item=EncodeLikeItem>,
		Items::IntoIter: ExactSizeIterator,
	{
		let encoded_value = Self::STORAGE_SPACE.get(&[0; 0])
			.unwrap_or_else(|| {
				match G::from_query_to_optional_value(G::from_optional_value_to_query(None)) {
					Some(value) => value.encode(),
					None => Vec::new(),
				}
			});

		let new_val = T::append_or_new(
			encoded_value,
			items,
		).map_err(|_| "Could not append given item")?;
		Self::STORAGE_SPACE.put(&[0; 0], &new_val);
		Ok(())
	}

	/// Safely append the given items to the value in the storage. If a codec error occurs, then the
	/// old (presumably corrupt) value is replaced with the given `items`.
	///
	/// `T` is required to implement `codec::EncodeAppend`.
	fn append_or_put<Items, Item, EncodeLikeItem>(items: Items) where
		Item: Encode,
		EncodeLikeItem: EncodeLike<Item>,
		T: EncodeAppend<Item=Item>,
		Items: IntoIterator<Item=EncodeLikeItem> + Clone + EncodeLike<T>,
		Items::IntoIter: ExactSizeIterator
	{
		Self::append(items.clone()).unwrap_or_else(|_| Self::put(items));
	}

	/// Read the length of the value in a fast way, without decoding the entire value.
	///
	/// `T` is required to implement `Codec::DecodeLength`.
	///
	/// Note that `0` is returned as the default value if no encoded value exists at the given key.
	/// Therefore, this function cannot be used as a sign of _existence_. use the `::exists()`
	/// function for this purpose.
	fn decode_len() -> Result<usize, &'static str> where T: codec::DecodeLength, T: Len {
		// attempt to get the length directly.
		if let Some(k) = Self::STORAGE_SPACE.get(&[0; 0]) {
			<T as codec::DecodeLength>::len(&k).map_err(|e| e.what())
		} else {
			let len = G::from_query_to_optional_value(G::from_optional_value_to_query(None))
				.map(|v| v.len())
				.unwrap_or(0);

			Ok(len)
		}
	}
}
