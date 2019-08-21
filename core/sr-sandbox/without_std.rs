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

use rstd::prelude::*;
use rstd::{slice, marker, mem, vec};
use rstd::rc::Rc;
use codec::{Decode, Encode};
use primitives::sandbox as sandbox_primitives;
use super::{Error, TypedValue, ReturnValue, HostFunc};

mod ffi {
	use rstd::mem;

	extern "C" {
		pub fn ext_sandbox_instantiate(
			wasm_ptr: *const u8,
			wasm_len: usize,
			imports_ptr: *const u8,
			imports_len: usize,
		) -> u32;
		pub fn ext_sandbox_invoke(
			instance_idx: u32,
			export_ptr: *const u8,
			export_len: usize,
			args_ptr: *const u8,
			args_len: usize,
			return_val_ptr: *mut u8,
			return_val_len: usize,
		) -> u32;
		pub fn ext_sandbox_memory_new(initial: u32, maximum: u32) -> u32;
		pub fn ext_sandbox_memory_get(
			memory_idx: u32,
			offset: u32,
			buf_ptr: *mut u8,
			buf_len: usize,
		) -> u32;
		pub fn ext_sandbox_memory_set(
			memory_idx: u32,
			offset: u32,
			val_ptr: *const u8,
			val_len: usize,
		) -> u32;
		pub fn ext_sandbox_memory_teardown(
			memory_idx: u32,
		);
		pub fn ext_sandbox_instance_teardown(
			instance_idx: u32,
		);
	}
}

struct MemoryHandle {
	memory_idx: u32,
}

impl Drop for MemoryHandle {
	fn drop(&mut self) {
		unsafe {
			ffi::ext_sandbox_memory_teardown(self.memory_idx);
		}
	}
}

#[derive(Clone)]
pub struct Memory {
	// Handle to memory instance is wrapped to add reference-counting semantics
	// to `Memory`.
	handle: Rc<MemoryHandle>,
}

impl Memory {
	pub fn new(initial: u32, maximum: Option<u32>) -> Result<Memory, Error> {
		let result = unsafe {
			let maximum = if let Some(maximum) = maximum {
				maximum
			} else {
				sandbox_primitives::MEM_UNLIMITED
			};
			ffi::ext_sandbox_memory_new(initial, maximum)
		};
		match result {
			sandbox_primitives::ERR_MODULE => Err(Error::Module),
			memory_idx => Ok(Memory {
				handle: Rc::new(MemoryHandle { memory_idx, }),
			}),
		}
	}

	pub fn get(&self, offset: u32, buf: &mut [u8]) -> Result<(), Error> {
		let result = unsafe { ffi::ext_sandbox_memory_get(self.handle.memory_idx, offset, buf.as_mut_ptr(), buf.len()) };
		match result {
			sandbox_primitives::ERR_OK => Ok(()),
			sandbox_primitives::ERR_OUT_OF_BOUNDS => Err(Error::OutOfBounds),
			_ => unreachable!(),
		}
	}

	pub fn set(&self, offset: u32, val: &[u8]) -> Result<(), Error> {
		let result = unsafe { ffi::ext_sandbox_memory_set(self.handle.memory_idx, offset, val.as_ptr(), val.len()) };
		match result {
			sandbox_primitives::ERR_OK => Ok(()),
			sandbox_primitives::ERR_OUT_OF_BOUNDS => Err(Error::OutOfBounds),
			_ => unreachable!(),
		}
	}
}

pub struct EnvironmentDefinitionBuilder {
	env_def: sandbox_primitives::EnvironmentDefinition,
	retained_memories: Vec<Memory>,
}

impl EnvironmentDefinitionBuilder {
	pub fn new() -> EnvironmentDefinitionBuilder {
		EnvironmentDefinitionBuilder {
			env_def: sandbox_primitives::EnvironmentDefinition {
				entries: Vec::new(),
			},
			retained_memories: Vec::new(),
		}
	}

	fn add_entry<N1, N2>(
		&mut self,
		module: N1,
		field: N2,
		extern_entity: sandbox_primitives::ExternEntity,
	) where
		N1: Into<Vec<u8>>,
		N2: Into<Vec<u8>>,
	{
		let entry = sandbox_primitives::Entry {
			module_name: module.into(),
			field_name: field.into(),
			entity: extern_entity,
		};
		self.env_def.entries.push(entry);
	}

	pub fn add_host_func<N1, N2, F>(&mut self, module: N1, field: N2, f: F)
	where
		N1: Into<Vec<u8>>,
		N2: Into<Vec<u8>>,
		F: HostFunc,
	{
		let f = sandbox_primitives::ExternEntity::Function(f.as_usize() as u32);
		self.add_entry(module, field, f);
	}

	pub fn add_memory<N1, N2>(&mut self, module: N1, field: N2, mem: Memory)
	where
		N1: Into<Vec<u8>>,
		N2: Into<Vec<u8>>,
	{
		// We need to retain memory to keep it alive while the EnvironmentDefinitionBuilder alive.
		self.retained_memories.push(mem.clone());

		let mem = sandbox_primitives::ExternEntity::Memory(mem.handle.memory_idx as u32);
		self.add_entry(module, field, mem);
	}
}

pub struct Instance {
	instance_idx: u32,
	_retained_memories: Vec<Memory>,
}

impl Instance {
	pub fn new(code: &[u8], env_def_builder: &EnvironmentDefinitionBuilder) -> Result<Instance, Error> {
		let serialized_env_def: Vec<u8> = env_def_builder.env_def.encode();
		let result = unsafe {
			ffi::ext_sandbox_instantiate(
				code.as_ptr(),
				code.len(),
				serialized_env_def.as_ptr(),
				serialized_env_def.len(),
			)
		};
		let instance_idx = match result {
			sandbox_primitives::ERR_MODULE => return Err(Error::Module),
			sandbox_primitives::ERR_EXECUTION => return Err(Error::Execution),
			instance_idx => instance_idx,
		};
		// We need to retain memories to keep them alive while the Instance is alive.
		let retained_memories = env_def_builder.retained_memories.clone();
		Ok(Instance {
			instance_idx,
			_retained_memories: retained_memories,
		})
	}

	pub fn invoke(
		&mut self,
		name: &[u8],
		args: &[TypedValue],
	) -> Result<ReturnValue, Error> {
		let serialized_args = args.to_vec().encode();
		let mut return_val = vec![0u8; sandbox_primitives::ReturnValue::ENCODED_MAX_SIZE];

		let result = unsafe {
			ffi::ext_sandbox_invoke(
				self.instance_idx,
				name.as_ptr(),
				name.len(),
				serialized_args.as_ptr(),
				serialized_args.len(),
				return_val.as_mut_ptr(),
				return_val.len(),
			)
		};
		match result {
			sandbox_primitives::ERR_OK => {
				let return_val = sandbox_primitives::ReturnValue::decode(&mut &return_val[..])
					.map_err(|_| Error::Execution)?;
				Ok(return_val)
			}
			sandbox_primitives::ERR_EXECUTION => Err(Error::Execution),
			_ => unreachable!(),
		}
	}
}

impl Drop for Instance {
	fn drop(&mut self) {
		unsafe {
			ffi::ext_sandbox_instance_teardown(self.instance_idx);
		}
	}
}
