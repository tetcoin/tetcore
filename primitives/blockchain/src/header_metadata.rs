// This file is part of Tetcore.

// Copyright (C) 2019-2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Implements tree backend, cached header metadata and algorithms
//! to compute routes efficiently over the tree of headers.

use tp_runtime::traits::{Block as BlockT, NumberFor, Header};
use parking_lot::RwLock;
use lru::LruCache;

/// Set to the expected max difference between `best` and `finalized` blocks at sync.
const LRU_CACHE_SIZE: usize = 5_000;

/// Get lowest common ancestor between two blocks in the tree.
///
/// This implementation is efficient because our trees have very few and
/// small branches, and because of our current query pattern:
/// lca(best, final), lca(best + 1, final), lca(best + 2, final), etc.
/// The first call is O(h) but the others are O(1).
pub fn lowest_common_ancestor<Block: BlockT, T: HeaderMetadata<Block> + ?Sized>(
	backend: &T,
	id_one: Block::Hash,
	id_two: Block::Hash,
) -> Result<HashAndNumber<Block>, T::Error> {
	let mut header_one = backend.header_metadata(id_one)?;
	let mut header_two = backend.header_metadata(id_two)?;

	let mut orig_header_one = header_one.clone();
	let mut orig_header_two = header_two.clone();

	// We move through ancestor links as much as possible, since ancestor >= parent.

	while header_one.number > header_two.number {
		let ancestor_one = backend.header_metadata(header_one.ancestor)?;

		if ancestor_one.number >= header_two.number {
			header_one = ancestor_one;
		} else {
			break
		}
	}

	while header_one.number < header_two.number {
		let ancestor_two = backend.header_metadata(header_two.ancestor)?;

		if ancestor_two.number >= header_one.number {
			header_two = ancestor_two;
		} else {
			break
		}
	}

	// Then we move the remaining path using parent links.

	while header_one.hash != header_two.hash {
		if header_one.number > header_two.number {
			header_one = backend.header_metadata(header_one.parent)?;
		} else {
			header_two = backend.header_metadata(header_two.parent)?;
		}
	}

	// Update cached ancestor links.

	if orig_header_one.number > header_one.number {
		orig_header_one.ancestor = header_one.hash;
		backend.insert_header_metadata(orig_header_one.hash, orig_header_one);
	}

	if orig_header_two.number > header_one.number {
		orig_header_two.ancestor = header_one.hash;
		backend.insert_header_metadata(orig_header_two.hash, orig_header_two);
	}

	Ok(HashAndNumber {
		hash: header_one.hash,
		number: header_one.number,
	})
}

/// Compute a tree-route between two blocks. See tree-route docs for more details.
pub fn tree_route<Block: BlockT, T: HeaderMetadata<Block>>(
	backend: &T,
	from: Block::Hash,
	to: Block::Hash,
) -> Result<TreeRoute<Block>, T::Error> {
	let mut from = backend.header_metadata(from)?;
	let mut to = backend.header_metadata(to)?;

	let mut from_branch = Vec::new();
	let mut to_branch = Vec::new();

	while to.number > from.number {
		to_branch.push(HashAndNumber {
			number: to.number,
			hash: to.hash,
		});

		to = backend.header_metadata(to.parent)?;
	}

	while from.number > to.number {
		from_branch.push(HashAndNumber {
			number: from.number,
			hash: from.hash,
		});
		from = backend.header_metadata(from.parent)?;
	}

	// numbers are equal now. walk backwards until the block is the same

	while to.hash != from.hash {
		to_branch.push(HashAndNumber {
			number: to.number,
			hash: to.hash,
		});
		to = backend.header_metadata(to.parent)?;

		from_branch.push(HashAndNumber {
			number: from.number,
			hash: from.hash,
		});
		from = backend.header_metadata(from.parent)?;
	}

	// add the pivot block. and append the reversed to-branch
	// (note that it's reverse order originals)
	let pivot = from_branch.len();
	from_branch.push(HashAndNumber {
		number: to.number,
		hash: to.hash,
	});
	from_branch.extend(to_branch.into_iter().rev());

	Ok(TreeRoute {
		route: from_branch,
		pivot,
	})
}

/// Hash and number of a block.
#[derive(Debug, Clone)]
pub struct HashAndNumber<Block: BlockT> {
	/// The number of the block.
	pub number: NumberFor<Block>,
	/// The hash of the block.
	pub hash: Block::Hash,
}

/// A tree-route from one block to another in the chain.
///
/// All blocks prior to the pivot in the deque is the reverse-order unique ancestry
/// of the first block, the block at the pivot index is the common ancestor,
/// and all blocks after the pivot is the ancestry of the second block, in
/// order.
///
/// The ancestry sets will include the given blocks, and thus the tree-route is
/// never empty.
///
/// ```text
/// Tree route from R1 to E2. Retracted is [R1, R2, R3], Common is C, enacted [E1, E2]
///   <- R3 <- R2 <- R1
///  /
/// C
///  \-> E1 -> E2
/// ```
///
/// ```text
/// Tree route from C to E2. Retracted empty. Common is C, enacted [E1, E2]
/// C -> E1 -> E2
/// ```
#[derive(Debug, Clone)]
pub struct TreeRoute<Block: BlockT> {
	route: Vec<HashAndNumber<Block>>,
	pivot: usize,
}

impl<Block: BlockT> TreeRoute<Block> {
	/// Get a slice of all retracted blocks in reverse order (towards common ancestor).
	pub fn retracted(&self) -> &[HashAndNumber<Block>] {
		&self.route[..self.pivot]
	}

	/// Convert into all retracted blocks in reverse order (towards common ancestor).
	pub fn into_retracted(mut self) -> Vec<HashAndNumber<Block>> {
		self.route.truncate(self.pivot);
		self.route
	}

	/// Get the common ancestor block. This might be one of the two blocks of the
	/// route.
	pub fn common_block(&self) -> &HashAndNumber<Block> {
		self.route.get(self.pivot).expect("tree-routes are computed between blocks; \
			which are included in the route; \
			thus it is never empty; qed")
	}

	/// Get a slice of enacted blocks (descendents of the common ancestor)
	pub fn enacted(&self) -> &[HashAndNumber<Block>] {
		&self.route[self.pivot + 1 ..]
	}
}

/// Handles header metadata: hash, number, parent hash, etc.
pub trait HeaderMetadata<Block: BlockT> {
	/// Error used in case the header metadata is not found.
	type Error;

	fn header_metadata(
		&self,
		hash: Block::Hash,
	) -> Result<CachedHeaderMetadata<Block>, Self::Error>;
	fn insert_header_metadata(
		&self,
		hash: Block::Hash,
		header_metadata: CachedHeaderMetadata<Block>,
	);
	fn remove_header_metadata(&self, hash: Block::Hash);
}

/// Caches header metadata in an in-memory LRU cache.
pub struct HeaderMetadataCache<Block: BlockT> {
	cache: RwLock<LruCache<Block::Hash, CachedHeaderMetadata<Block>>>,
}

impl<Block: BlockT> HeaderMetadataCache<Block> {
	/// Creates a new LRU header metadata cache with `capacity`.
	pub fn new(capacity: usize) -> Self {
		HeaderMetadataCache {
			cache: RwLock::new(LruCache::new(capacity)),
		}
	}
}

impl<Block: BlockT> Default for HeaderMetadataCache<Block> {
	fn default() -> Self {
		HeaderMetadataCache {
			cache: RwLock::new(LruCache::new(LRU_CACHE_SIZE)),
		}
	}
}

impl<Block: BlockT> HeaderMetadataCache<Block> {
	pub fn header_metadata(&self, hash: Block::Hash) -> Option<CachedHeaderMetadata<Block>> {
		self.cache.write().get(&hash).cloned()
	}

	pub fn insert_header_metadata(&self, hash: Block::Hash, metadata: CachedHeaderMetadata<Block>) {
		self.cache.write().put(hash, metadata);
	}

	pub fn remove_header_metadata(&self, hash: Block::Hash) {
		self.cache.write().pop(&hash);
	}
}

/// Cached header metadata. Used to efficiently traverse the tree.
#[derive(Debug, Clone)]
pub struct CachedHeaderMetadata<Block: BlockT> {
	/// Hash of the header.
	pub hash: Block::Hash,
	/// Block number.
	pub number: NumberFor<Block>,
	/// Hash of parent header.
	pub parent: Block::Hash,
	/// Block state root.
	pub state_root: Block::Hash,
	/// Hash of an ancestor header. Used to jump through the tree.
	ancestor: Block::Hash,
}

impl<Block: BlockT> From<&Block::Header> for CachedHeaderMetadata<Block> {
	fn from(header: &Block::Header) -> Self {
		CachedHeaderMetadata {
			hash: header.hash().clone(),
			number: header.number().clone(),
			parent: header.parent_hash().clone(),
			state_root: header.state_root().clone(),
			ancestor: header.parent_hash().clone(),
		}
	}
}
