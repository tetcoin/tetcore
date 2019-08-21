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

use rstd::collections::btree_map::BTreeMap;
use rstd::fmt;

use wasmi::{
	Externals, FuncInstance, FuncRef, GlobalDescriptor, GlobalRef, ImportResolver,
	MemoryDescriptor, MemoryInstance, MemoryRef, Module, ModuleInstance, ModuleRef,
	RuntimeArgs, RuntimeValue, Signature, TableDescriptor, TableRef, Trap, TrapKind
};
use wasmi::memory_units::Pages;
use super::{Error, TypedValue, ReturnValue, HostError, HostFunc, UntypedHostFunc};

#[derive(Clone)]
pub struct Memory {
	memref: MemoryRef,
}

impl Memory {
	pub fn new(initial: u32, maximum: Option<u32>) -> Result<Memory, Error> {
		Ok(Memory {
			memref: MemoryInstance::alloc(
				Pages(initial as usize),
				maximum.map(|m| Pages(m as usize)),
			).map_err(|_| Error::Module)?,
		})
	}

	pub fn get(&self, ptr: u32, buf: &mut [u8]) -> Result<(), Error> {
		self.memref.get_into(ptr, buf).map_err(|_| Error::OutOfBounds)?;
		Ok(())
	}

	pub fn set(&self, ptr: u32, value: &[u8]) -> Result<(), Error> {
		self.memref.set(ptr, value).map_err(|_| Error::OutOfBounds)?;
		Ok(())
	}
}

struct HostFuncIndex(usize);

#[derive(Clone)]
struct DefinedHostFunctions {
	funcs: Vec<UntypedHostFunc>,
}

impl DefinedHostFunctions {
	fn new() -> DefinedHostFunctions {
		DefinedHostFunctions {
			funcs: Vec::new(),
		}
	}

	fn define(&mut self, f: UntypedHostFunc) -> HostFuncIndex {
		let idx = self.funcs.len();
		self.funcs.push(f);
		HostFuncIndex(idx)
	}
}

#[derive(Debug)]
struct DummyHostError;

impl fmt::Display for DummyHostError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "DummyHostError")
	}
}

impl wasmi::HostError for DummyHostError {
}

fn from_runtime_value(v: RuntimeValue) -> TypedValue {
	match v {
		RuntimeValue::I32(v) => TypedValue::I32(v),
		RuntimeValue::I64(v) => TypedValue::I64(v),
		RuntimeValue::F32(v) => TypedValue::F32(v.to_bits() as i32),
		RuntimeValue::F64(v) => TypedValue::F64(v.to_bits() as i64),
	}
}

fn to_runtime_value(v: TypedValue) -> RuntimeValue {
	use wasmi::nan_preserving_float::{F32, F64};
	match v {
		TypedValue::I32(v) => RuntimeValue::I32(v as i32),
		TypedValue::I64(v) => RuntimeValue::I64(v as i64),
		TypedValue::F32(v_bits) => RuntimeValue::F32(F32::from_bits(v_bits as u32)),
		TypedValue::F64(v_bits) => RuntimeValue::F64(F64::from_bits(v_bits as u64)),
	}
}

struct GuestExternals<'a> {
	defined_host_functions: &'a DefinedHostFunctions,
}

impl<'a> Externals for GuestExternals<'a> {
	fn invoke_index(
		&mut self,
		index: usize,
		args: RuntimeArgs,
	) -> Result<Option<RuntimeValue>, Trap> {
		let args = args.as_ref()
			.iter()
			.cloned()
			.map(from_runtime_value)
			.collect::<Vec<_>>();

		let result = self.defined_host_functions.funcs[index].call(&args);
		match result {
			Ok(value) => Ok(match value {
				ReturnValue::Value(v) => Some(to_runtime_value(v)),
				ReturnValue::Unit => None,
			}),
			Err(HostError) => Err(TrapKind::Host(Box::new(DummyHostError)).into()),
		}
	}
}

enum ExternVal {
	HostFunc(HostFuncIndex),
	Memory(Memory),
}

pub struct EnvironmentDefinitionBuilder {
	map: BTreeMap<(Vec<u8>, Vec<u8>), ExternVal>,
	defined_host_functions: DefinedHostFunctions,
}

impl EnvironmentDefinitionBuilder {
	pub fn new() -> EnvironmentDefinitionBuilder {
		EnvironmentDefinitionBuilder {
			map: BTreeMap::new(),
			defined_host_functions: DefinedHostFunctions::new(),
		}
	}

	pub fn add_host_func<N1, N2, F>(&mut self, module: N1, field: N2, f: F)
	where
		N1: Into<Vec<u8>>,
		N2: Into<Vec<u8>>,
		F: HostFunc,
	{
		let untyped_fn = f.untype();
		let idx = self.defined_host_functions.define(untyped_fn);
		self.map
			.insert((module.into(), field.into()), ExternVal::HostFunc(idx));
	}

	pub fn add_memory<N1, N2>(&mut self, module: N1, field: N2, mem: Memory)
	where
		N1: Into<Vec<u8>>,
		N2: Into<Vec<u8>>,
	{
		self.map
			.insert((module.into(), field.into()), ExternVal::Memory(mem));
	}
}

impl ImportResolver for EnvironmentDefinitionBuilder {
	fn resolve_func(
		&self,
		module_name: &str,
		field_name: &str,
		signature: &Signature,
	) -> Result<FuncRef, wasmi::Error> {
		let key = (
			module_name.as_bytes().to_owned(),
			field_name.as_bytes().to_owned(),
		);
		let externval = self.map.get(&key).ok_or_else(|| {
			wasmi::Error::Instantiation(format!("Export {}:{} not found", module_name, field_name))
		})?;
		let host_func_idx = match *externval {
			ExternVal::HostFunc(ref idx) => idx,
			_ => {
				return Err(wasmi::Error::Instantiation(format!(
					"Export {}:{} is not a host func",
					module_name, field_name
				)))
			}
		};
		Ok(FuncInstance::alloc_host(signature.clone(), host_func_idx.0))
	}

	fn resolve_global(
		&self,
		_module_name: &str,
		_field_name: &str,
		_global_type: &GlobalDescriptor,
	) -> Result<GlobalRef, wasmi::Error> {
		Err(wasmi::Error::Instantiation(format!(
			"Importing globals is not supported yet"
		)))
	}

	fn resolve_memory(
		&self,
		module_name: &str,
		field_name: &str,
		_memory_type: &MemoryDescriptor,
	) -> Result<MemoryRef, wasmi::Error> {
		let key = (
			module_name.as_bytes().to_owned(),
			field_name.as_bytes().to_owned(),
		);
		let externval = self.map.get(&key).ok_or_else(|| {
			wasmi::Error::Instantiation(format!("Export {}:{} not found", module_name, field_name))
		})?;
		let memory = match *externval {
			ExternVal::Memory(ref m) => m,
			_ => {
				return Err(wasmi::Error::Instantiation(format!(
					"Export {}:{} is not a memory",
					module_name, field_name
				)))
			}
		};
		Ok(memory.memref.clone())
	}

	fn resolve_table(
		&self,
		_module_name: &str,
		_field_name: &str,
		_table_type: &TableDescriptor,
	) -> Result<TableRef, wasmi::Error> {
		Err(wasmi::Error::Instantiation(format!(
			"Importing tables is not supported yet"
		)))
	}
}

pub struct Instance {
	instance: ModuleRef,
	defined_host_functions: DefinedHostFunctions,
}

impl Instance {
	pub fn new(code: &[u8], env_def_builder: &EnvironmentDefinitionBuilder) -> Result<Instance, Error> {
		let module = Module::from_buffer(code).map_err(|_| Error::Module)?;
		let not_started_instance = ModuleInstance::new(&module, env_def_builder)
			.map_err(|_| Error::Module)?;

		let defined_host_functions = env_def_builder.defined_host_functions.clone();
		let instance = {
			let mut externals = GuestExternals {
				defined_host_functions: &defined_host_functions,
			};
			let instance = not_started_instance.run_start(&mut externals).map_err(|_| Error::Execution)?;
			instance
		};

		Ok(Instance {
			instance,
			defined_host_functions,
		})
	}

	pub fn invoke(
		&mut self,
		name: &[u8],
		args: &[TypedValue],
	) -> Result<ReturnValue, Error> {
		let args = args.iter().cloned().map(Into::into).collect::<Vec<_>>();

		let name = ::std::str::from_utf8(name).map_err(|_| Error::Execution)?;
		let mut externals = GuestExternals {
			defined_host_functions: &self.defined_host_functions,
		};
		let result = self.instance
			.invoke_export(&name, &args, &mut externals);

		match result {
			Ok(None) => Ok(ReturnValue::Unit),
			Ok(Some(val)) => Ok(ReturnValue::Value(val.into())),
			Err(_err) => Err(Error::Execution),
		}
	}
}

#[cfg(test)]
mod tests {
	use std::thread_local;
	use std::cell::RefCell;
	use wabt;
	use crate::{Error, TypedValue, ReturnValue, HostError, EnvironmentDefinitionBuilder, Instance};
	use assert_matches::assert_matches;

	fn execute_sandboxed(code: &[u8], args: &[TypedValue]) -> Result<ReturnValue, HostError> {
		struct State {
			counter: u32,
		}

		thread_local! {
			static STATE: RefCell<State> = RefCell::new(State { counter: 0 });
		}

		fn env_assert(condition: u32) -> Result<(), HostError> {
			if condition != 0 {
				Ok(())
			} else {
				Err(HostError)
			}
		}
		fn env_inc_counter(inc_by: u32) -> Result<u32, HostError> {
			STATE.with(|state| {
				let mut state = state.borrow_mut();
				state.counter += inc_by as u32;
				Ok(state.counter)
			})
		}

		let mut env_builder = EnvironmentDefinitionBuilder::new();
		env_builder.add_host_func("env", "assert", env_assert as fn(_) -> _);
		env_builder.add_host_func("env", "inc_counter", env_inc_counter as fn(_) -> _);

		let mut instance = Instance::new(code, &env_builder)?;
		let result = instance.invoke(b"call", args);

		result.map_err(|_| HostError)
	}

	#[test]
	fn invoke_args() {
		let code = wabt::wat2wasm(r#"
		(module
			(import "env" "assert" (func $assert (param i32)))

			(func (export "call") (param $x i32) (param $y i64)
				;; assert that $x = 0x12345678
				(call $assert
					(i32.eq
						(get_local $x)
						(i32.const 0x12345678)
					)
				)

				(call $assert
					(i64.eq
						(get_local $y)
						(i64.const 0x1234567887654321)
					)
				)
			)
		)
		"#).unwrap();

		let result = execute_sandboxed(
			&code,
			&[
				TypedValue::I32(0x12345678),
				TypedValue::I64(0x1234567887654321),
			]
		);
		assert!(result.is_ok());
	}

	#[test]
	fn return_value() {
		let code = wabt::wat2wasm(r#"
		(module
			(func (export "call") (param $x i32) (result i32)
				(i32.add
					(get_local $x)
					(i32.const 1)
				)
			)
		)
		"#).unwrap();

		let return_val = execute_sandboxed(
			&code,
			&[
				TypedValue::I32(0x1336),
			]
		).unwrap();
		assert_eq!(return_val, ReturnValue::Value(TypedValue::I32(0x1337)));
	}

	#[test]
	fn cant_return_unmatching_type() {
		fn env_returns_i32() -> Result<u32, HostError> {
			Ok(42)
		}

		let mut env_builder = EnvironmentDefinitionBuilder::new();
		env_builder.add_host_func("env", "returns_i32", env_returns_i32 as fn() -> _);

		let code = wabt::wat2wasm(r#"
		(module
			;; It's actually returns i32, but imported as if it returned i64
			(import "env" "returns_i32" (func $returns_i32 (result i64)))

			(func (export "call")
				(drop
					(call $returns_i32)
				)
			)
		)
		"#).unwrap();

		// It succeeds since we are able to import functions with types we want.
		let mut instance = Instance::new(&code, &env_builder).unwrap();

		// But this fails since we imported a function that returns i32 as if it returned i64.
		assert_matches!(
			instance.invoke(b"call", &[]),
			Err(Error::Execution)
		);
	}
}
