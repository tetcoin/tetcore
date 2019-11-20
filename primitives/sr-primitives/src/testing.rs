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

//! Testing utilities.

use serde::{Serialize, Serializer, Deserialize, de::Error as DeError, Deserializer};
use std::{fmt::Debug, ops::Deref, fmt, cell::RefCell};
use crate::codec::{Codec, Encode, Decode};
use crate::traits::{
	self, Checkable, Applyable, BlakeTwo256, OpaqueKeys,
	SignedExtension, Dispatchable,
};
#[allow(deprecated)]
use crate::traits::ValidateUnsigned;
use crate::{generic, KeyTypeId, ApplyResult};
use crate::weights::{GetDispatchInfo, DispatchInfo};
pub use primitives::{H256, sr25519};
use primitives::{crypto::{CryptoType, Dummy, key_types, Public}, U256};
use crate::transaction_validity::{TransactionValidity, TransactionValidityError};

/// Authority Id
#[derive(Default, PartialEq, Eq, Clone, Encode, Decode, Debug, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub struct UintAuthorityId(pub u64);

impl From<u64> for UintAuthorityId {
	fn from(id: u64) -> Self {
		UintAuthorityId(id)
	}
}

impl From<UintAuthorityId> for u64 {
	fn from(id: UintAuthorityId) -> u64 {
		id.0
	}
}

impl AsRef<[u8]> for UintAuthorityId {
	fn as_ref(&self) -> &[u8] {
		// Unsafe, i know, but it's test code and it's just there because it's really convenient to
		// keep `UintAuthorityId` as a u64 under the hood.
		unsafe {
			std::slice::from_raw_parts(&self.0 as *const u64 as *const _, std::mem::size_of::<u64>())
		}
	}
}

impl AsMut<[u8]> for UintAuthorityId {
	fn as_mut(&mut self) -> &mut [u8] {
		unsafe {
			std::slice::from_raw_parts_mut(&mut self.0 as *mut u64 as *mut _, std::mem::size_of::<u64>())
		}
	}
}



thread_local! {
	/// A list of all UintAuthorityId keys returned to the runtime.
	static ALL_KEYS: RefCell<Vec<UintAuthorityId>> = RefCell::new(vec![]);
}

impl UintAuthorityId {
	/// Set the list of keys returned by the runtime call for all keys of that type.
	pub fn set_all_keys<T: Into<UintAuthorityId>>(keys: impl IntoIterator<Item=T>) {
		ALL_KEYS.with(|l| *l.borrow_mut() = keys.into_iter().map(Into::into).collect())
	}

	/// Convert this authority id into a public key.
	pub fn to_public_key<T: Public>(&self) -> T {
		let bytes: [u8; 32] = U256::from(self.0).into();
		T::from_slice(&bytes)
	}
}

impl app_crypto::RuntimeAppPublic for UintAuthorityId {
	const ID: KeyTypeId = key_types::DUMMY;

	type Signature = Self;
	type Generic = Self;

	fn all() -> Vec<Self> {
		ALL_KEYS.with(|l| l.borrow().clone())
	}

	fn generate_pair(_: Option<Vec<u8>>) -> Self {
		use rand::RngCore;
		UintAuthorityId(rand::thread_rng().next_u64())
	}

	fn sign<M: AsRef<[u8]>>(&self, msg: &M) -> Option<Self::Signature> {
		let mut signature = [0u8; 8];
		msg.as_ref().iter()
			.chain(std::iter::repeat(&42u8))
			.take(8)
			.enumerate()
			.for_each(|(i, v)| { signature[i] = *v; });

		Some(Self(u64::from_le_bytes(signature)))
	}

	fn verify<M: AsRef<[u8]>>(&self, msg: &M, signature: &Self::Signature) -> bool {
		let mut msg_signature = [0u8; 8];
		msg.as_ref().iter()
			.chain(std::iter::repeat(&42))
			.take(8)
			.enumerate()
			.for_each(|(i, v)| { msg_signature[i] = *v; });

		u64::from_le_bytes(msg_signature) == signature.0
	}
}

impl OpaqueKeys for UintAuthorityId {
	type KeyTypeIdProviders = ();

	fn key_ids() -> &'static [KeyTypeId] {
		&[key_types::DUMMY]
	}

	fn get_raw(&self, _: KeyTypeId) -> &[u8] {
		self.as_ref()
	}

	fn get<T: Decode>(&self, _: KeyTypeId) -> Option<T> {
		self.using_encoded(|mut x| T::decode(&mut x)).ok()
	}
}

impl crate::BoundToRuntimeAppPublic for UintAuthorityId {
	type Public = Self;
}

impl app_crypto::AppPublic for UintAuthorityId {
	type Generic = Self;
}

impl app_crypto::AppKey for UintAuthorityId {
	/// An identifier for this application-specific key type.
	const ID: KeyTypeId = primitives::crypto::key_types::DUMMY;

	/// The corresponding type as a generic crypto type.
	type UntypedGeneric = Self;

	/// The corresponding public key type in this application scheme.
	type Public = Self;

	/// The corresponding key pair type in this application scheme.
	// #[cfg(feature = "full_crypto")]
	type Pair = Self;

	/// The corresponding signature type in this application scheme.
	type Signature = Self;
}

impl app_crypto::AppPair for UintAuthorityId {
	type Generic = Self;
}

impl app_crypto::AppSignature for UintAuthorityId {
	type Generic = Self;
}

impl From<Dummy> for UintAuthorityId {
	fn from(_: Dummy) -> Self {
		Self(11)
	}
}

impl AsRef<Self> for UintAuthorityId {
	fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<Self> for UintAuthorityId {
	fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl primitives::crypto::Public for UintAuthorityId {
	fn from_slice(data: &[u8]) -> Self {
		// TODO make better
		Self(data[0] as u64)
	}
}

impl primitives::crypto::Derive for UintAuthorityId {}

impl CryptoType for UintAuthorityId {
	type Pair = Dummy;
}

impl crate::traits::IdentifyAccount for UintAuthorityId {
	type AccountId = u64;
	fn into_account(self) -> Self::AccountId {
		self.0
	}
}

impl crate::traits::IdentifyAccount for Dummy {
	type AccountId = u64;
	fn into_account(self) -> Self::AccountId {
		11
	}
}

impl primitives::crypto::Pair for UintAuthorityId {
		type Public = Self;
		type Seed = Self;
		type Signature = Self;
		type DeriveError = ();
		#[cfg(feature = "std")]
		fn generate_with_phrase(_: Option<&str>) -> (Self, String, Self::Seed) { Default::default() }
		#[cfg(feature = "std")]
		fn from_phrase(_: &str, _: Option<&str>)
			-> Result<(Self, Self::Seed), primitives::crypto::SecretStringError>
		{
			Ok(Default::default())
		}
		fn derive<
			Iter: Iterator<Item=primitives::crypto::DeriveJunction>,
		>(&self, _: Iter, _: Option<Self>) -> Result<(Self, Option<Self>), Self::DeriveError> { Err(()) }
		fn from_seed(_: &Self::Seed) -> Self { Self(11) }
		fn from_seed_slice(_: &[u8]) -> Result<Self, primitives::crypto::SecretStringError> { Ok(Self(11)) }
		fn sign(&self, _: &[u8]) -> Self::Signature { Self(self.0) }
		fn verify<M: AsRef<[u8]>>(_: &Self::Signature, _: M, _: &Self::Public) -> bool { true }
		fn verify_weak<P: AsRef<[u8]>, M: AsRef<[u8]>>(_: &[u8], _: M, _: P) -> bool { true }
		fn public(&self) -> Self::Public { Self::Public::from(self.0) }
		fn to_raw_vec(&self) -> Vec<u8> { vec![self.0 as u8] }
	}

/// Digest item
pub type DigestItem = generic::DigestItem<H256>;

/// Header Digest
pub type Digest = generic::Digest<H256>;

/// Block Header
#[derive(PartialEq, Eq, Clone, Serialize, Debug, Encode, Decode, Default)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Header {
	/// Parent hash
	pub parent_hash: H256,
	/// Block Number
	pub number: u64,
	/// Post-execution state trie root
	pub state_root: H256,
	/// Merkle root of block's extrinsics
	pub extrinsics_root: H256,
	/// Digest items
	pub digest: Digest,
}

impl traits::Header for Header {
	type Number = u64;
	type Hashing = BlakeTwo256;
	type Hash = H256;

	fn number(&self) -> &Self::Number { &self.number }
	fn set_number(&mut self, num: Self::Number) { self.number = num }

	fn extrinsics_root(&self) -> &Self::Hash { &self.extrinsics_root }
	fn set_extrinsics_root(&mut self, root: Self::Hash) { self.extrinsics_root = root }

	fn state_root(&self) -> &Self::Hash { &self.state_root }
	fn set_state_root(&mut self, root: Self::Hash) { self.state_root = root }

	fn parent_hash(&self) -> &Self::Hash { &self.parent_hash }
	fn set_parent_hash(&mut self, hash: Self::Hash) { self.parent_hash = hash }

	fn digest(&self) -> &Digest { &self.digest }
	fn digest_mut(&mut self) -> &mut Digest { &mut self.digest }

	fn new(
		number: Self::Number,
		extrinsics_root: Self::Hash,
		state_root: Self::Hash,
		parent_hash: Self::Hash,
		digest: Digest,
	) -> Self {
		Header {
			number,
			extrinsics_root,
			state_root,
			parent_hash,
			digest,
		}
	}
}

impl Header {
	/// A new header with the given number and default hash for all other fields.
	pub fn new_from_number(number: <Self as traits::Header>::Number) -> Self {
		Self {
			number,
			..Default::default()
		}
	}
}

impl<'a> Deserialize<'a> for Header {
	fn deserialize<D: Deserializer<'a>>(de: D) -> Result<Self, D::Error> {
		let r = <Vec<u8>>::deserialize(de)?;
		Decode::decode(&mut &r[..])
			.map_err(|e| DeError::custom(format!("Invalid value passed into decode: {}", e.what())))
	}
}

/// An opaque extrinsic wrapper type.
#[derive(PartialEq, Eq, Clone, Debug, Encode, Decode)]
pub struct ExtrinsicWrapper<Xt>(Xt);

impl<Xt> traits::Extrinsic for ExtrinsicWrapper<Xt> {
	type Call = ();
	type SignaturePayload = ();

	fn is_signed(&self) -> Option<bool> {
		None
	}
}

impl<Xt: Encode> serde::Serialize for ExtrinsicWrapper<Xt> {
	fn serialize<S>(&self, seq: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
		self.using_encoded(|bytes| seq.serialize_bytes(bytes))
	}
}

impl<Xt> From<Xt> for ExtrinsicWrapper<Xt> {
	fn from(xt: Xt) -> Self {
		ExtrinsicWrapper(xt)
	}
}

impl<Xt> Deref for ExtrinsicWrapper<Xt> {
	type Target = Xt;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

/// Testing block
#[derive(PartialEq, Eq, Clone, Serialize, Debug, Encode, Decode)]
pub struct Block<Xt> {
	/// Block header
	pub header: Header,
	/// List of extrinsics
	pub extrinsics: Vec<Xt>,
}

impl<Xt: 'static + Codec + Sized + Send + Sync + Serialize + Clone + Eq + Debug + traits::Extrinsic> traits::Block
	for Block<Xt>
{
	type Extrinsic = Xt;
	type Header = Header;
	type Hash = <Header as traits::Header>::Hash;

	fn header(&self) -> &Self::Header {
		&self.header
	}
	fn extrinsics(&self) -> &[Self::Extrinsic] {
		&self.extrinsics[..]
	}
	fn deconstruct(self) -> (Self::Header, Vec<Self::Extrinsic>) {
		(self.header, self.extrinsics)
	}
	fn new(header: Self::Header, extrinsics: Vec<Self::Extrinsic>) -> Self {
		Block { header, extrinsics }
	}
	fn encode_from(header: &Self::Header, extrinsics: &[Self::Extrinsic]) -> Vec<u8> {
		(header, extrinsics).encode()
	}
}

impl<'a, Xt> Deserialize<'a> for Block<Xt> where Block<Xt>: Decode {
	fn deserialize<D: Deserializer<'a>>(de: D) -> Result<Self, D::Error> {
		let r = <Vec<u8>>::deserialize(de)?;
		Decode::decode(&mut &r[..])
			.map_err(|e| DeError::custom(format!("Invalid value passed into decode: {}", e.what())))
	}
}

/// Test transaction, tuple of (sender, call, signed_extra)
/// with index only used if sender is some.
///
/// If sender is some then the transaction is signed otherwise it is unsigned.
#[derive(PartialEq, Eq, Clone, Encode, Decode)]
pub struct TestXt<Call, Extra>(pub Option<(u64, Extra)>, pub Call);

impl<Call, Extra> Serialize for TestXt<Call, Extra> where TestXt<Call, Extra>: Encode {
	fn serialize<S>(&self, seq: S) -> Result<S::Ok, S::Error> where S: Serializer {
		self.using_encoded(|bytes| seq.serialize_bytes(bytes))
	}
}

impl<Call, Extra> Debug for TestXt<Call, Extra> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "TestXt({:?}, ...)", self.0.as_ref().map(|x| &x.0))
	}
}

impl<Call: Codec + Sync + Send, Context, Extra> Checkable<Context> for TestXt<Call, Extra> {
	type Checked = Self;
	fn check(self, _: &Context) -> Result<Self::Checked, TransactionValidityError> { Ok(self) }
}
impl<Call: Codec + Sync + Send, Extra> traits::Extrinsic for TestXt<Call, Extra> {
	type Call = Call;
	type SignaturePayload = (u64, Extra);

	fn is_signed(&self) -> Option<bool> {
		Some(self.0.is_some())
	}

	fn new(c: Call, sig: Option<Self::SignaturePayload>) -> Option<Self> {
		Some(TestXt(sig, c))
	}
}

impl<Origin, Call, Extra> Applyable for TestXt<Call, Extra> where
	Call: 'static + Sized + Send + Sync + Clone + Eq + Codec + Debug + Dispatchable<Origin=Origin>,
	Extra: SignedExtension<AccountId=u64, Call=Call>,
	Origin: From<Option<u64>>,
{
	type AccountId = u64;
	type Call = Call;

	fn sender(&self) -> Option<&Self::AccountId> { self.0.as_ref().map(|x| &x.0) }

	/// Checks to see if this is a valid *transaction*. It returns information on it if so.
	#[allow(deprecated)] // Allow ValidateUnsigned
	fn validate<U: ValidateUnsigned<Call=Self::Call>>(
		&self,
		_info: DispatchInfo,
		_len: usize,
	) -> TransactionValidity {
		Ok(Default::default())
	}

	/// Executes all necessary logic needed prior to dispatch and deconstructs into function call,
	/// index and sender.
	#[allow(deprecated)] // Allow ValidateUnsigned
	fn apply<U: ValidateUnsigned<Call=Self::Call>>(
		self,
		info: DispatchInfo,
		len: usize,
	) -> ApplyResult {
		let maybe_who = if let Some((who, extra)) = self.0 {
			Extra::pre_dispatch(extra, &who, &self.1, info, len)?;
			Some(who)
		} else {
			Extra::pre_dispatch_unsigned(&self.1, info, len)?;
			None
		};

		Ok(self.1.dispatch(maybe_who.into()).map_err(Into::into))
	}
}

impl<Call: Encode, Extra: Encode> GetDispatchInfo for TestXt<Call, Extra> {
	fn get_dispatch_info(&self) -> DispatchInfo {
		// for testing: weight == size.
		DispatchInfo {
			weight: self.encode().len() as _,
			..Default::default()
		}
	}
}
