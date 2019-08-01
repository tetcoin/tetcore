// Copyright 2018-2019 Parity Technologies (UK) Ltd.
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

//! Low-level types used throughout the Substrate code.

#![warn(missing_docs)]

#![cfg_attr(not(feature = "std"), no_std)]

use sr_primitives::{
	generic, traits::{Verify, BlakeTwo256}, OpaqueExtrinsic, AnySignature
};

/// An index to a block.
pub type BlockNumber = u64;

/// Alias to 512-bit hash when used in the context of a transaction signature on the chain.
pub type Signature = AnySignature;

/// Some way of identifying an account on the chain. We intentionally make it equivalent
/// to the public key of our transaction signing scheme.
pub type AccountId = <Signature as Verify>::Signer;

/// The type for looking up accounts. We don't expect more than 4 billion of them, but you
/// never know...
pub type AccountIndex = u32;

use sr_primitives::traits::*;
use rstd::convert::*;
use rstd::ops::*;


#[derive(Clone, PartialEq, Eq, Debug, Ord, PartialOrd, Default, Copy, Decode, Encode)]
pub struct I128(
	#[codec(encoded_as(u128))]
	i128
);

impl Zero for I128 {
	fn is_zero(&self) -> bool {
		self.0.is_zero()
	}
	fn zero() -> Self {
		I128(0)
	}
}
impl One for I128 {
	fn one() -> Self {
		I128(0)
	}
}
impl IntegerSquareRoot for I128 {
	fn integer_sqrt_checked(&self) -> Option<Self> {
		self.0.integer_sqrt_checked().map(|r| I128(r))
	}
}
impl From<u8> for I128 {
	fn from(x: u8) -> Self { I128(x as i128) }
}
impl From<u16> for I128 {
	fn from(x: u16) -> Self { I128(x as i128) }
}
impl From<u32> for I128 {
	fn from(x: u32) -> Self { I128(x as i128) }
}
impl From<u64> for I128 {
	fn from(x: u64) -> Self { I128(x as i128) }
}
impl TryInto<u8> for I128 {
	type Error = rstd::num::TryFromIntError;
	fn try_into(self) -> Result<u8, Self::Error> {
		self.0.try_into()
	}
}
impl TryInto<u16> for I128 {
	type Error = rstd::num::TryFromIntError;
	fn try_into(self) -> Result<u16, Self::Error> {
		self.0.try_into()
	}
}
impl TryInto<u32> for I128 {
	type Error = rstd::num::TryFromIntError;
	fn try_into(self) -> Result<u32, Self::Error> {
		self.0.try_into()
	}
}
impl TryInto<u64> for I128 {
	type Error = rstd::num::TryFromIntError;
	fn try_into(self) -> Result<u64, Self::Error> {
		self.0.try_into()
	}
}
impl TryInto<u128> for I128 {
	type Error = rstd::num::TryFromIntError;
	fn try_into(self) -> Result<u128, Self::Error> {
		self.0.try_into()
	}
}
impl TryInto<usize> for I128 {
	type Error = rstd::num::TryFromIntError;
	fn try_into(self) -> Result<usize, Self::Error> {
		self.0.try_into()
	}
}
impl TryFrom<u128> for I128 {
	type Error = rstd::num::TryFromIntError;
	fn try_from(value: u128) -> Result<Self, Self::Error> {
		Ok(I128(i128::try_from(value)?))
	}
}
impl TryFrom<usize> for I128 {
	type Error = rstd::num::TryFromIntError;
	fn try_from(value: usize) -> Result<Self, Self::Error> {
		Ok(I128(i128::try_from(value)?))
	}
}
impl Add for I128 {
	type Output = Self;
	fn add(self, rhs: Self) -> Self::Output {
		I128(self.0 + rhs.0)
	}
}
impl AddAssign<Self> for I128 {
	fn add_assign(&mut self, rhs: Self) {
		*self = I128(self.0 + rhs.0)
	}
}
impl Sub<Self> for I128 {
	type Output = Self;
	fn sub(self, rhs: Self) -> Self::Output {
		I128(self.0 - rhs.0)
	}
}
impl SubAssign<Self> for I128 {
	fn sub_assign(&mut self, rhs: Self) {
		*self = I128(self.0 - rhs.0)
	}
}
impl Mul<Self> for I128 {
	type Output = Self;
	fn mul(self, rhs: Self) -> Self::Output {
		I128(self.0 * rhs.0)
	}
}
impl MulAssign<Self> for I128 {
	fn mul_assign(&mut self, rhs: Self) {
		*self = I128(self.0 * rhs.0)
	}
}
impl Div<Self> for I128 {
	type Output = Self;
	fn div(self, rhs: Self) -> Self::Output {
		I128(self.0 / rhs.0)
	}
}
impl DivAssign<Self> for I128 {
	fn div_assign(&mut self, rhs: Self) {
		*self = I128(self.0 / rhs.0)
	}
}
impl Rem<Self> for I128 {
	type Output = Self;
	fn rem(self, rhs: Self) -> Self::Output {
		I128(self.0 % rhs.0)
	}
}
impl RemAssign<Self> for I128 {
	fn rem_assign(&mut self, rhs: Self) {
		*self = I128(self.0 / rhs.0)
	}
}
impl Shl<u32> for I128 {
	type Output = Self;
	fn shl(self, rhs: u32) -> Self::Output {
		I128(self.0.shl(rhs))
	}
}
impl Shr<u32> for I128 {
	type Output = Self;
	fn shr(self, rhs: u32) -> Self::Output {
		I128(self.0.shr(rhs))
	}
}
impl CheckedShl for I128 {
	fn checked_shl(&self, rhs: u32) -> Option<Self> {
		Some(I128(self.0.checked_shl(rhs)?))
	}
}
impl CheckedShr for I128 {
	fn checked_shr(&self, rhs: u32) -> Option<Self> {
		Some(I128(self.0.checked_shr(rhs)?))
	}
}
impl CheckedAdd for I128 {
	fn checked_add(&self, rhs: &Self) -> Option<Self> {
		Some(I128(self.0.checked_add(rhs.0)?))
	}
}
impl CheckedSub for I128 {
	fn checked_sub(&self, rhs: &Self) -> Option<Self> {
		Some(I128(self.0.checked_sub(rhs.0)?))
	}
}
impl CheckedMul for I128 {
	fn checked_mul(&self, rhs: &Self) -> Option<Self> {
		Some(I128(self.0.checked_mul(rhs.0)?))
	}
}
impl CheckedDiv for I128 {
	fn checked_div(&self, rhs: &Self) -> Option<Self> {
		Some(I128(self.0.checked_div(rhs.0)?))
	}
}
impl Saturating for I128 {
	fn saturating_add(self, o: Self) -> Self {
		I128(self.0.saturating_add(o.0))
	}
	fn saturating_sub(self, o: Self) -> Self {
		I128(self.0.saturating_sub(o.0))
	}
	fn saturating_mul(self, o: Self) -> Self {
		I128(self.0.saturating_mul(o.0))
	}
}
impl Bounded for I128 {
	fn min_value() -> Self {
		I128(i128::min_value())
	}
	fn max_value() -> Self {
		I128(i128::max_value())
	}
}
use parity_codec::*;
impl HasCompact for I128 {
	type Type = Compact<u128>;
}
impl From<I128> for Compact<u128> {
	fn from(x: I128) -> Self{
		Compact(x.0 as u128)
	}
}
impl From<Compact<u128>> for I128 {
	fn from(x: Compact<u128>) -> Self {
		I128(x.0 as i128)
	}
}
impl<'a> EncodeAsRef<'a, I128> for Compact<u128> {
	type RefType = MyCompactRef<'a>;
}

pub struct MyCompactRef<'a>(&'a i128);
impl<'a> Encode for MyCompactRef<'a> {
	fn encode(&self) -> rstd::vec::Vec<u8> {
		Compact(*self.0 as u128).encode()
	}
}

impl<'a> From<&'a I128> for MyCompactRef<'a> {
	fn from(x: &'a I128) -> Self {
		MyCompactRef(&x.0)
	}
}

/// Balance of an account.
pub type Balance = I128;

/// Type used for expressing timestamp.
pub type Moment = u64;

/// Index of a transaction in the chain.
pub type Index = u64;

/// A hash of some data used by the chain.
pub type Hash = primitives::H256;

/// A timestamp: milliseconds since the unix epoch.
/// `u64` is enough to represent a duration of half a billion years, when the
/// time scale is milliseconds.
pub type Timestamp = u64;

/// Digest item type.
pub type DigestItem = generic::DigestItem<Hash>;
/// Header type.
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
/// Block type.
pub type Block = generic::Block<Header, UncheckedExtrinsic>;
/// Block ID.
pub type BlockId = generic::BlockId<Block>;

/// Opaque, encoded, unchecked extrinsic.
pub type UncheckedExtrinsic = OpaqueExtrinsic;
