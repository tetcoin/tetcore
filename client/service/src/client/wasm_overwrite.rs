// This file is part of Substrate.

// Copyright (C) 2017-2020 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.
use std::{ 
    fs, collections::{HashMap, hash_map::DefaultHasher}, path::Path,
    hash::Hasher as _, sync::Arc, 
};
use codec::{Encode, Decode};
use sp_core::{
    traits::FetchRuntimeCode,
};
use sp_state_machine::BasicExternalities;
use sc_executor::RuntimeInfo;
use sp_runtime::traits::Block as BlockT;
use sp_version::RuntimeVersion;
use sp_core::traits::RuntimeCode;

#[derive(Clone, Debug)]
struct WasmBlob {
    code: Vec<u8>,
}

impl WasmBlob {
    fn new(code: Vec<u8>) -> Self {
        Self { code }
    }

    fn runtime_code(&self) -> RuntimeCode {
        RuntimeCode {
            code_fetcher: self,
            heap_pages: Some(128),
            hash: make_hash(self.code.as_slice()).to_le_bytes().encode()
        }
    }
}

/// Make a hash out of a byte string using the default hasher
pub fn make_hash<K: std::hash::Hash + ?Sized>(val: &K) -> u64 {
    let mut state = DefaultHasher::new();
    val.hash(&mut state);
    state.finish()
}

impl FetchRuntimeCode for WasmBlob {
    fn fetch_runtime_code<'a>(&'a self) -> Option<std::borrow::Cow<'a, [u8]>> {
        Some(self.code.as_slice().into())
    }
}

#[derive(Clone, Debug)]
pub struct WasmOverwrite {
    // Map of runtime spec version -> Wasm Blob
    overwrites: HashMap<u32, WasmBlob>,
}

impl WasmOverwrite {
    pub fn new<P: AsRef<Path>, E: RuntimeInfo + Clone + 'static>(path: Option<P>, executor: &E) -> sp_blockchain::Result<Self> {
        let overwrites = if let Some(path) = path {
            scrape_overwrites(path.as_ref(), executor)?
        } else {
            HashMap::new()
        };
        
        Ok(Self { overwrites })
    }
}

/// Scrapes a folder for WASM runtimes.
/// Gets the version from the runtime
fn scrape_overwrites<E: RuntimeInfo + Clone + 'static>(dir: &Path, executor: &E) -> sp_blockchain::Result<HashMap<u32, WasmBlob>> {  
    // instantiate host functions
    // basic externalities
    // runtime version
    let handle_err = |e: std::io::Error | -> sp_blockchain::Error {
        sp_blockchain::Error::Msg(format!("{}", e.to_string()))
    };
    
    let mut overwrites = HashMap::new(); 
    if dir.is_dir() {
        for entry in fs::read_dir(dir).map_err(handle_err)? {
            let entry = entry.map_err(handle_err)?;
            let path = entry.path();
            let wasm = WasmBlob::new(fs::read(path).map_err(handle_err)?);
            let version = runtime_version(executor, &wasm)?;
            overwrites.insert(version.spec_version, wasm);
        }
    }
    Ok(overwrites)
}

fn runtime_version<E: RuntimeInfo + Clone + 'static>(executor: &E, code: &WasmBlob) -> sp_blockchain::Result<RuntimeVersion> {
    let mut ext = BasicExternalities::default(); 
    executor.runtime_version(&mut ext, &code.runtime_code())
        .map_err(|e| sp_blockchain::Error::VersionInvalid(format!("{:?}", e)).into())
}

#[cfg(test)]
mod test {
    use super::*;    
    use sc_executor::{NativeExecutor, WasmExecutionMethod};
    use sc_executor::sp_wasm_interface::HostFunctions;
    
    #[test]
    fn should_get_runtime_version() {
        let host_functions = sp_io::SubstrateHostFunctions::host_functions();
        let exec = NativeExecutor::<substrate_test_runtime_client::LocalExecutor>::new(WasmExecutionMethod::Interpreted, Some(128), 1);
        let overwrites = WasmOverwrite::new(Some("/home/insipx/wasm"), &exec).unwrap();
        for key in overwrites.overwrites.keys() {
            println!("{}", key);
        }
    }
}
