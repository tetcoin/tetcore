// Copyright 2017-2019 Parity Technologies (UK) Ltd.
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

use sr_primitives::traits::{Block as BlockT, DigestItemFor, Header as HeaderT, NumberFor};
use sr_primitives::Justification;
use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::Arc;

use crate::import_queue::{Verifier, CacheKeyId};

pub trait BlockAnnounce<B: BlockT> {
	fn announce_block(&self, hash: B::Hash, data: Vec<u8>);
}
