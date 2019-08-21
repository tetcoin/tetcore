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

//! This crate provides means to instantiate and execute wasm modules.
//!
//! It works even when the user of this library executes from
//! inside the wasm VM. In this case the same VM is used for execution
//! of both the sandbox owner and the sandboxed module, without compromising security
//! and without the performance penalty of full wasm emulation inside wasm.
//!
//! This is achieved by using bindings to the wasm VM, which are published by the host API.
//! This API is thin and consists of only a handful functions. It contains functions for instantiating
//! modules and executing them, but doesn't contain functions for inspecting the module
//! structure. The user of this library is supposed to read the wasm module.
//!
//! When this crate is used in the `std` environment all these functions are implemented by directly
//! calling the wasm VM.
//!
//! Examples of possible use-cases for this library are not limited to the following:
//!
//! - implementing smart-contract runtimes that use wasm for contract code
//! - executing a wasm substrate runtime inside of a wasm parachain

#![warn(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]

use rstd::prelude::*;

// TODO: Replace with rstd
#[cfg(feature = "std")]
use std::any::TypeId;




pub use primitives::sandbox::{Signature, ValueType, TypedValue, ReturnValue, HostError};

mod imp {
	#[cfg(feature = "std")]
	include!("../with_std.rs");

	#[cfg(not(feature = "std"))]
	include!("../without_std.rs");
}

/// Error that can occur while using this crate.
#[cfg_attr(feature = "std", derive(Debug))]
pub enum Error {
	/// Module is not valid, couldn't be instantiated.
	Module,

	/// Access to a memory or table was made with an address or an index which is out of bounds.
	///
	/// Note that if wasm module makes an out-of-bounds access then trap will occur.
	OutOfBounds,

	/// Failed to invoke the start function or an exported function for some reason.
	Execution,
}

impl From<Error> for HostError {
	fn from(_e: Error) -> HostError {
		HostError
	}
}

/// Reference to a sandboxed linear memory, that
/// will be used by the guest module.
///
/// The memory can't be directly accessed by supervisor, but only
/// through designated functions [`get`] and [`set`].
///
/// [`get`]: #method.get
/// [`set`]: #method.set
#[derive(Clone)]
pub struct Memory {
	inner: imp::Memory,
}

impl Memory {
	/// Construct a new linear memory instance.
	///
	/// The memory allocated with initial number of pages specified by `initial`.
	/// Minimal possible value for `initial` is 0 and maximum possible is `65536`.
	/// (Since maximum addressable memory is 2<sup>32</sup> = 4GiB = 65536 * 64KiB).
	///
	/// It is possible to limit maximum number of pages this memory instance can have by specifying
	/// `maximum`. If not specified, this memory instance would be able to allocate up to 4GiB.
	///
	/// Allocated memory is always zeroed.
	pub fn new(initial: u32, maximum: Option<u32>) -> Result<Memory, Error> {
		Ok(Memory {
			inner: imp::Memory::new(initial, maximum)?,
		})
	}

	/// Read a memory area at the address `ptr` with the size of the provided slice `buf`.
	///
	/// Returns `Err` if the range is out-of-bounds.
	pub fn get(&self, ptr: u32, buf: &mut [u8]) -> Result<(), Error> {
		self.inner.get(ptr, buf)
	}

	/// Write a memory area at the address `ptr` with contents of the provided slice `buf`.
	///
	/// Returns `Err` if the range is out-of-bounds.
	pub fn set(&self, ptr: u32, value: &[u8]) -> Result<(), Error> {
		self.inner.set(ptr, value)
	}
}

// TODO: Move it

pub trait WasmReturnType {
	const WASM_TYPE: Option<ValueType>;
	fn into_return_value(self) -> ReturnValue;
}

impl WasmReturnType for () {
	const WASM_TYPE: Option<ValueType> = None;
	fn into_return_value(self) -> ReturnValue {
		ReturnValue::Unit
	}
}

impl WasmReturnType for u32 {
	const WASM_TYPE: Option<ValueType> = Some(ValueType::I32);
	fn into_return_value(self) -> ReturnValue {
		ReturnValue::Value(TypedValue::I32(self as i32))
	}
}

// TODO: Impls for i32, u64, i64, f32, f64

pub trait WasmParamType: Sized {
	const WASM_TYPE: ValueType;
	fn from_typed_value(v: TypedValue) -> Option<Self>;
}

impl WasmParamType for u32 {
	const WASM_TYPE: ValueType = ValueType::I32;
	fn from_typed_value(v: TypedValue) -> Option<Self> {
		match v {
			TypedValue::I32(i) => Some(i as u32),
			_ => None,
		}
	}
}

// TODO: Impls for i32, u64, i64, f32, f64

// TODO: Rename to WasmParamTypes?
pub trait WasmParams {
	const WASM_TYPES: &'static [ValueType];
}

impl WasmParams for () {
	const WASM_TYPES: &'static [ValueType] = &[];
}

impl<A: WasmParamType> WasmParams for (A,) {
	const WASM_TYPES: &'static [ValueType] = &[A::WASM_TYPE];
}

impl<A: WasmParamType, B: WasmParamType> WasmParams for (A, B) {
	const WASM_TYPES: &'static [ValueType] = &[A::WASM_TYPE, B::WASM_TYPE];
}

// TODO: Impls for tuples

#[inline]
pub fn signature_matches<F: HostFunc>(sig: &Signature) -> bool {
	&*sig.param_tys == <F::ParamTypes as WasmParams>::WASM_TYPES
		&& sig.return_ty == <F::ReturnType as WasmReturnType>::WASM_TYPE
}

pub fn signature_of<F: HostFunc>() -> Signature {
	Signature {
		param_tys: <F::ParamTypes as WasmParams>::WASM_TYPES.to_vec(),
		return_ty: <F::ReturnType as WasmReturnType>::WASM_TYPE.clone(),
	}
}

// TODO: Maybe move this into without_std? Maybe not, to simplify macro generation

pub trait HostFunc {
	type ReturnType: WasmReturnType;
	type ParamTypes: WasmParams;

	// TODO: Rename -> as fn index.
	fn as_usize(&self) -> usize;

	#[cfg(feature = "std")]
	fn untype(&self) -> UntypedHostFunc;
}

impl<R: WasmReturnType + 'static> HostFunc for fn() -> Result<R, HostError> {
	type ReturnType = R;
	type ParamTypes = ();

	fn as_usize(&self) -> usize {
		*self as *const () as usize
	}

	#[cfg(feature = "std")]
	fn untype(&self) -> UntypedHostFunc {
		let dispatcher =
			|untyped_fn: &UntypedHostFunc, _args: &[TypedValue]| -> Result<ReturnValue, HostError> {
				let f: Self = untyped_fn.reify().unwrap();
				(f)().map(|r| r.into_return_value())
			};
		UntypedHostFunc::from(*self, dispatcher).unwrap()
	}
}

impl<R: WasmReturnType + 'static, A: WasmParamType + 'static> HostFunc for fn(A) -> Result<R, HostError> {
	type ReturnType = R;
	type ParamTypes = (A,);

	fn as_usize(&self) -> usize {
		*self as *const () as usize
	}

	#[cfg(feature = "std")]
	fn untype(&self) -> UntypedHostFunc {
		let dispatcher =
			|untyped_fn: &UntypedHostFunc, args: &[TypedValue]| -> Result<ReturnValue, HostError> {
				let f: Self = untyped_fn.reify().unwrap();

				let arg0 = A::from_typed_value(args.get(0).cloned().unwrap()).unwrap();
				(f)(arg0).map(|r| r.into_return_value())
			};
		UntypedHostFunc::from(*self, dispatcher).unwrap()
	}
}

impl<R: WasmReturnType + 'static, A: WasmParamType + 'static, B: WasmParamType + 'static> HostFunc
	for fn(A, B) -> Result<R, HostError>
{
	type ReturnType = R;
	type ParamTypes = (A, B);

	fn as_usize(&self) -> usize {
		*self as *const () as usize
	}

	#[cfg(feature = "std")]
	fn untype(&self) -> UntypedHostFunc {
		let dispatcher =
			|untyped_fn: &UntypedHostFunc, args: &[TypedValue]| -> Result<ReturnValue, HostError> {
				let f: Self = untyped_fn.reify().unwrap();

				let arg0 = A::from_typed_value(args.get(0).cloned().unwrap()).unwrap();
				let arg1 = B::from_typed_value(args.get(1).cloned().unwrap()).unwrap();
				(f)(arg0, arg1).map(|r| r.into_return_value())
			};
		UntypedHostFunc::from(*self, dispatcher).unwrap()
	}
}

/// Internally this type is similar to `Any`, however, unlike `Any` this type is `Sized`.
#[cfg(feature = "std")]
#[derive(Clone)]
pub struct UntypedHostFunc {
	raw_fn_addr: usize,
	type_id: TypeId,
	dispatcher: fn(&UntypedHostFunc, args: &[TypedValue]) -> Result<ReturnValue, HostError>,
}

#[cfg(feature = "std")]
impl UntypedHostFunc {
	fn from<F: Copy + 'static>(
		f: F,
		dispatcher: fn(&UntypedHostFunc, args: &[TypedValue]) -> Result<ReturnValue, HostError>,
	) -> Option<Self> {
		if rstd::mem::size_of::<F>() == rstd::mem::size_of::<usize>() {
			let raw_fn_addr = unsafe {
				// This is safe since `F` and `usize` has the same sizes.
				rstd::mem::transmute_copy::<F, usize>(&f)
			};
			let type_id = TypeId::of::<F>();
			Some(UntypedHostFunc {
				raw_fn_addr,
				type_id,
				dispatcher,
			})
		} else {
			None
		}
	}

	fn reify<F: Sized + Copy + 'static>(&self) -> Option<F> {
		if TypeId::of::<F>() == self.type_id {
			let f = unsafe {
				// This should be safe since the original type of `raw_fn_addr` is the same as `F`
				// and thus the sizes are equal. The type is `Copy` so it is safe to copy it out.
				rstd::mem::transmute_copy::<usize, F>(&self.raw_fn_addr)
			};
			Some(f)
		} else {
			None
		}
	}

	fn call(&self, args: &[TypedValue]) -> Result<ReturnValue, HostError> {
		(self.dispatcher)(self, args)
	}
}

/// Struct that can be used for defining an environment for a sandboxed module.
///
/// The sandboxed module can access only the entities which were defined and passed
/// to the module at the instantiation time.
pub struct EnvironmentDefinitionBuilder {
	inner: imp::EnvironmentDefinitionBuilder,
}

impl EnvironmentDefinitionBuilder {
	/// Construct a new `EnvironmentDefinitionBuilder`.
	pub fn new() -> EnvironmentDefinitionBuilder {
		EnvironmentDefinitionBuilder {
			inner: imp::EnvironmentDefinitionBuilder::new(),
		}
	}

	/// Register a host function in this environment definition.
	///
	/// NOTE that there is no constraints on type of this function. An instance
	/// can import function passed here with any signature it wants. It can even import
	/// the same function (i.e. with same `module` and `field`) several times. It's up to
	/// the user code to check or constrain the types of signatures.
	pub fn add_host_func<N1, N2, F>(&mut self, module: N1, field: N2, f: F)
	where
		N1: Into<Vec<u8>>,
		N2: Into<Vec<u8>>,
		F: HostFunc,
	{
		self.inner.add_host_func(module, field, f);
	}

	/// Register a memory in this environment definition.
	pub fn add_memory<N1, N2>(&mut self, module: N1, field: N2, mem: Memory)
	where
		N1: Into<Vec<u8>>,
		N2: Into<Vec<u8>>,
	{
		self.inner.add_memory(module, field, mem.inner);
	}
}

/// Sandboxed instance of a wasm module.
///
/// This instance can be used for invoking exported functions.
pub struct Instance {
	inner: imp::Instance,
}

impl Instance {
	/// Instantiate a module with the given [`EnvironmentDefinitionBuilder`]. It will
	/// run the `start` function.
	///
	/// Returns `Err(Error::Module)` if this module can't be instantiated with the given
	/// environment. If execution of `start` function generated a trap, then `Err(Error::Execution)` will
	/// be returned.
	///
	/// [`EnvironmentDefinitionBuilder`]: struct.EnvironmentDefinitionBuilder.html
	pub fn new(code: &[u8], env_def_builder: &EnvironmentDefinitionBuilder) -> Result<Instance, Error> {
		Ok(Instance {
			inner: imp::Instance::new(code, &env_def_builder.inner)?,
		})
	}

	/// Invoke an exported function with the given name.
	///
	/// # Errors
	///
	/// Returns `Err(Error::Execution)` if:
	///
	/// - An export function name isn't a proper utf8 byte sequence,
	/// - This module doesn't have an exported function with the given name,
	/// - If types of the arguments passed to the function doesn't match function signature
	///   then trap occurs (as if the exported function was called via call_indirect),
	/// - Trap occured at the execution time.
	pub fn invoke(
		&mut self,
		name: &[u8],
		args: &[TypedValue],
	) -> Result<ReturnValue, Error> {
		self.inner.invoke(name, args)
	}
}
