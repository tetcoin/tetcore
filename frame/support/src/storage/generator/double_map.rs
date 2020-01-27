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

use sp_std::prelude::*;
use sp_std::borrow::Borrow;
use codec::{Ref, FullCodec, FullEncode, Encode, EncodeLike, EncodeAppend};
use crate::{storage::{self, unhashed, StorageSpaceIteratorDecode, storage_space::StorageSpace}, hash::{StorageHasher, Twox128}, traits::Len};

/// Generator for `StorageDoubleMap` used by `decl_storage`.
///
/// # Mapping of keys to a storage path
///
/// The storage key (i.e. the key under which the `Value` will be stored) is created from two parts.
/// The first part is a hash of a concatenation of the `key1_prefix` and `Key1`. And the second part
/// is a hash of a `Key2`.
///
/// Thus value for (key1, key2) is stored at:
/// ```nocompile
/// Twox128(module_prefix) ++ Twox128(storage_prefix) ++ Hasher1(encode(key1)) ++ Hasher2(encode(key2))
/// ```
///
/// # Warning
///
/// If the key1s are not trusted (e.g. can be set by a user), a cryptographic `hasher` such as
/// `blake2_256` must be used for Hasher1. Otherwise, other values in storage can be compromised.
/// If the key2s are not trusted (e.g. can be set by a user), a cryptographic `hasher` such as
/// `blake2_256` must be used for Hasher2. Otherwise, other items in storage with the same first
/// key can be compromised.
pub trait StorageDoubleMap<K1: FullEncode, K2: FullEncode, V: FullCodec> {
	/// The type that get/take returns.
	type Query;

	/// Hasher for the first key.
	type Hasher1: StorageHasher;

	/// Hasher for the second key.
	type Hasher2: StorageHasher;

	/// The space used in the trie to store its information. This space must not collide with any
	/// other storage space.
	const STORAGE_SPACE: Self::StorageSpace;

	/// The type of the storage space used.
	type StorageSpace: StorageSpace;

	/// Convert an optional value retrieved from storage to the type queried.
	fn from_optional_value_to_query(v: Option<V>) -> Self::Query;

	/// Convert a query to an optional value into storage.
	fn from_query_to_optional_value(v: Self::Query) -> Option<V>;

	/// Generate the first part of the key used in storage space.
	// TODO TODO: rename as it behavior has changed
	// ALSO return output instead of Vec for key1
	fn storage_double_map_final_key1<KArg1>(k1: KArg1) -> Vec<u8>
	where
		KArg1: EncodeLike<K1>,
	{
		k1.borrow().using_encoded(Self::Hasher1::hash).as_ref().to_vec()
	}

	/// Generate the full key used in storage space.
	// TODO TODO: rename as it behavior has changed
	fn storage_double_map_final_key<KArg1, KArg2>(k1: KArg1, k2: KArg2) -> Vec<u8>
	where
		KArg1: EncodeLike<K1>,
		KArg2: EncodeLike<K2>,
	{
		let mut final_key = Self::storage_double_map_final_key1(k1);
		final_key.extend_from_slice(k2.using_encoded(Self::Hasher2::hash).as_ref());
		final_key
	}
}

impl<K1, K2, V, G> storage::StorageDoubleMap<K1, K2, V> for G
where
	K1: FullEncode,
	K2: FullEncode,
	V: FullCodec,
	G: StorageDoubleMap<K1, K2, V>,
{
	type Query = G::Query;

	// TODO TODO: remove or change name
	fn hashed_key_for<KArg1, KArg2>(k1: KArg1, k2: KArg2) -> Vec<u8>
	where
		KArg1: EncodeLike<K1>,
		KArg2: EncodeLike<K2>,
	{
		unimplemented!();
		// Self::storage_double_map_final_key(k1, k2)
	}

	fn exists<KArg1, KArg2>(k1: KArg1, k2: KArg2) -> bool
	where
		KArg1: EncodeLike<K1>,
		KArg2: EncodeLike<K2>,
	{
		Self::STORAGE_SPACE.exists(&Self::storage_double_map_final_key(k1, k2))
	}

	fn get<KArg1, KArg2>(k1: KArg1, k2: KArg2) -> Self::Query
	where
		KArg1: EncodeLike<K1>,
		KArg2: EncodeLike<K2>,
	{
		G::from_optional_value_to_query(
			Self::STORAGE_SPACE.get_decode_and_warn(&Self::storage_double_map_final_key(k1, k2))
		)
	}

	fn take<KArg1, KArg2>(k1: KArg1, k2: KArg2) -> Self::Query
	where
		KArg1: EncodeLike<K1>,
		KArg2: EncodeLike<K2>,
	{
		let final_key = Self::storage_double_map_final_key(k1, k2);

		let value = Self::STORAGE_SPACE.get_decode_and_warn(&final_key);
		if value.is_some() {
			Self::STORAGE_SPACE.kill(&final_key);
		}
		G::from_optional_value_to_query(value)
	}

	fn swap<XKArg1, XKArg2, YKArg1, YKArg2>(x_k1: XKArg1, x_k2: XKArg2, y_k1: YKArg1, y_k2: YKArg2)
	where
		XKArg1: EncodeLike<K1>,
		XKArg2: EncodeLike<K2>,
		YKArg1: EncodeLike<K1>,
		YKArg2: EncodeLike<K2>
	{
		let final_x_key = Self::storage_double_map_final_key(x_k1, x_k2);
		let final_y_key = Self::storage_double_map_final_key(y_k1, y_k2);

		let v1 = Self::STORAGE_SPACE.get(&final_x_key);
		if let Some(val) = Self::STORAGE_SPACE.get(&final_y_key) {
			Self::STORAGE_SPACE.put(&final_x_key, &val);
		} else {
			Self::STORAGE_SPACE.kill(&final_x_key)
		}
		if let Some(val) = v1 {
			Self::STORAGE_SPACE.put(&final_y_key, &val);
		} else {
			Self::STORAGE_SPACE.kill(&final_y_key)
		}
	}

	fn insert<KArg1, KArg2, VArg>(k1: KArg1, k2: KArg2, val: VArg)
	where
		KArg1: EncodeLike<K1>,
		KArg2: EncodeLike<K2>,
		VArg: EncodeLike<V>,
	{
		val.using_encoded(|val| {
			Self::STORAGE_SPACE.put(&Self::storage_double_map_final_key(k1, k2), val)
		})
	}

	fn remove<KArg1, KArg2>(k1: KArg1, k2: KArg2)
	where
		KArg1: EncodeLike<K1>,
		KArg2: EncodeLike<K2>,
	{
		Self::STORAGE_SPACE.kill(&Self::storage_double_map_final_key(k1, k2))
	}

	fn remove_prefix<KArg1>(k1: KArg1) where KArg1: EncodeLike<K1> {
		Self::STORAGE_SPACE.kill_prefix(Self::storage_double_map_final_key1(k1).as_ref())
	}

	type IterPrefix = StorageSpaceIteratorDecode<
		<<Self as StorageDoubleMap<K1, K2, V>>::StorageSpace as StorageSpace>::StorageSpaceIterator, V
	>;


	fn iter_prefix<KArg1>(k1: KArg1) -> Self::IterPrefix
		where KArg1: ?Sized + EncodeLike<K1>
	{
		let prefix = Self::storage_double_map_final_key1(k1);
		StorageSpaceIteratorDecode {
			storage_space_iterator: Self::STORAGE_SPACE.iter_prefix(&prefix),
			phantom_data: Default::default(),
		}
	}

	fn mutate<KArg1, KArg2, R, F>(k1: KArg1, k2: KArg2, f: F) -> R
	where
		KArg1: EncodeLike<K1>,
		KArg2: EncodeLike<K2>,
		F: FnOnce(&mut Self::Query) -> R,
	{
		let final_key = Self::storage_double_map_final_key(k1, k2);
		let mut val = G::from_optional_value_to_query(
			Self::STORAGE_SPACE.get_decode_and_warn(final_key.as_ref())
		);

		let ret = f(&mut val);
		match G::from_query_to_optional_value(val) {
			Some(ref val) => val.using_encoded(|val| {
				Self::STORAGE_SPACE.put(final_key.as_ref(), val)
			}),
			None => Self::STORAGE_SPACE.kill(final_key.as_ref()),
		}
		ret
	}

	fn append<Items, Item, EncodeLikeItem, KArg1, KArg2>(
		k1: KArg1,
		k2: KArg2,
		items: Items,
	) -> Result<(), &'static str>
	where
		KArg1: EncodeLike<K1>,
		KArg2: EncodeLike<K2>,
		Item: Encode,
		EncodeLikeItem: EncodeLike<Item>,
		V: EncodeAppend<Item=Item>,
		Items: IntoIterator<Item=EncodeLikeItem>,
		Items::IntoIter: ExactSizeIterator
	{
		let final_key = Self::storage_double_map_final_key(k1, k2);

		let encoded_value = Self::STORAGE_SPACE.get(&final_key)
			.unwrap_or_else(|| {
				match G::from_query_to_optional_value(G::from_optional_value_to_query(None)) {
					Some(value) => value.encode(),
					None => Vec::new(),
				}
			});

		let new_val = V::append_or_new(
			encoded_value,
			items,
		).map_err(|_| "Could not append given item")?;
		Self::STORAGE_SPACE.put(&final_key, &new_val);

		Ok(())
	}

	fn append_or_insert<Items, Item, EncodeLikeItem, KArg1, KArg2>(
		k1: KArg1,
		k2: KArg2,
		items: Items,
	)
	where
		KArg1: EncodeLike<K1>,
		KArg2: EncodeLike<K2>,
		Item: Encode,
		EncodeLikeItem: EncodeLike<Item>,
		V: EncodeAppend<Item=Item>,
		Items: IntoIterator<Item=EncodeLikeItem> + Clone + EncodeLike<V>,
		Items::IntoIter: ExactSizeIterator
	{
		Self::append(Ref::from(&k1), Ref::from(&k2), items.clone())
			.unwrap_or_else(|_| Self::insert(k1, k2, items));
	}

	fn decode_len<KArg1, KArg2>(key1: KArg1, key2: KArg2) -> Result<usize, &'static str>
		where KArg1: EncodeLike<K1>,
		      KArg2: EncodeLike<K2>,
		      V: codec::DecodeLength + Len,
	{
		let final_key = Self::storage_double_map_final_key(key1, key2);
		if let Some(v) = Self::STORAGE_SPACE.get(&final_key) {
			<V as codec::DecodeLength>::len(&v).map_err(|e| e.what())
		} else {
			let len = G::from_query_to_optional_value(G::from_optional_value_to_query(None))
				.map(|v| v.len())
				.unwrap_or(0);

			Ok(len)
		}
	}
}

#[cfg(test)]
mod test {
	use sp_io::TestExternalities;
	use crate::storage::{self, StorageDoubleMap, storage_space};
	use crate::hash::Twox128;

	#[test]
	fn iter_prefix_works() {
		TestExternalities::default().execute_with(|| {
			struct MyStorage;
			impl storage::generator::StorageDoubleMap<u64, u64, u64> for MyStorage {
				type Query = Option<u64>;
				const STORAGE_SPACE: Self::StorageSpace = storage_space::PrefixedTopTrie {
					prefix0: &[49, 231, 135, 44, 86, 252, 191, 89, 93, 98, 118, 2, 239, 90, 126, 194],
					prefix1: &[1, 117, 91, 77, 227, 34, 193, 250, 189, 173, 243, 195, 23, 6, 109, 19],
				};
				type StorageSpace = storage_space::PrefixedTopTrie;
				type Hasher1 = Twox128;
				type Hasher2 = Twox128;
				fn from_optional_value_to_query(v: Option<u64>) -> Self::Query { v }
				fn from_query_to_optional_value(v: Self::Query) -> Option<u64> { v }
			}

			MyStorage::insert(1, 3, 7);
			MyStorage::insert(1, 4, 8);
			MyStorage::insert(2, 5, 9);
			MyStorage::insert(2, 6, 10);

			assert_eq!(MyStorage::iter_prefix(1).collect::<Vec<_>>(), vec![7, 8]);
			assert_eq!(MyStorage::iter_prefix(2).collect::<Vec<_>>(), vec![10, 9]);
		});
	}
}
