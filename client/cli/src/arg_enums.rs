// This file is part of Tetcore.

// Copyright (C) 2018-2021 Parity Technologies (UK) Ltd.
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
// NOTE: we allow missing docs here because arg_enum! creates the function variants without doc
#![allow(missing_docs)]

use structopt::clap::arg_enum;

arg_enum! {
	/// How to execute Wasm runtime code
	#[allow(missing_docs)]
	#[derive(Debug, Clone, Copy)]
	pub enum WasmExecutionMethod {
		// Uses an interpreter.
		Interpreted,
		// Uses a compiled runtime.
		Compiled,
	}
}

impl WasmExecutionMethod {
	/// Returns list of variants that are not disabled by feature flags.
	pub fn enabled_variants() -> Vec<&'static str> {
		Self::variants()
			.iter()
			.cloned()
			.filter(|&name| cfg!(feature = "wasmtime") || name != "Compiled")
			.collect()
	}
}

impl Into<tc_service::config::WasmExecutionMethod> for WasmExecutionMethod {
	fn into(self) -> tc_service::config::WasmExecutionMethod {
		match self {
			WasmExecutionMethod::Interpreted => {
				tc_service::config::WasmExecutionMethod::Interpreted
			}
			#[cfg(feature = "wasmtime")]
			WasmExecutionMethod::Compiled => tc_service::config::WasmExecutionMethod::Compiled,
			#[cfg(not(feature = "wasmtime"))]
			WasmExecutionMethod::Compiled => panic!(
				"Tetcore must be compiled with \"wasmtime\" feature for compiled Wasm execution"
			),
		}
	}
}

arg_enum! {
	#[allow(missing_docs)]
	#[derive(Debug, Copy, Clone, PartialEq, Eq)]
	pub enum TracingReceiver {
		Log,
		Telemetry,
	}
}

impl Into<tc_tracing::TracingReceiver> for TracingReceiver {
	fn into(self) -> tc_tracing::TracingReceiver {
		match self {
			TracingReceiver::Log => tc_tracing::TracingReceiver::Log,
			TracingReceiver::Telemetry => tc_tracing::TracingReceiver::Telemetry,
		}
	}
}

arg_enum! {
	#[allow(missing_docs)]
	#[derive(Debug, Copy, Clone, PartialEq, Eq)]
	pub enum NodeKeyType {
		Ed25519
	}
}

arg_enum! {
	#[derive(Debug, Copy, Clone, PartialEq, Eq)]
	pub enum CryptoScheme {
		Ed25519,
		Sr25519,
		Ecdsa,
	}
}

arg_enum! {
	#[derive(Debug, Copy, Clone, PartialEq, Eq)]
	pub enum OutputType {
		Json,
		Text,
	}
}

arg_enum! {
	/// How to execute blocks
	#[derive(Debug, Clone, Copy, PartialEq, Eq)]
	pub enum ExecutionStrategy {
		// Execute with native build (if available, WebAssembly otherwise).
		Native,
		// Only execute with the WebAssembly build.
		Wasm,
		// Execute with both native (where available) and WebAssembly builds.
		Both,
		// Execute with the native build if possible; if it fails, then execute with WebAssembly.
		NativeElseWasm,
	}
}

impl Into<tc_client_api::ExecutionStrategy> for ExecutionStrategy {
	fn into(self) -> tc_client_api::ExecutionStrategy {
		match self {
			ExecutionStrategy::Native => tc_client_api::ExecutionStrategy::NativeWhenPossible,
			ExecutionStrategy::Wasm => tc_client_api::ExecutionStrategy::AlwaysWasm,
			ExecutionStrategy::Both => tc_client_api::ExecutionStrategy::Both,
			ExecutionStrategy::NativeElseWasm => tc_client_api::ExecutionStrategy::NativeElseWasm,
		}
	}
}

impl ExecutionStrategy {
	/// Returns the variant as `'&static str`.
	pub fn as_str(&self) -> &'static str {
		match self {
			Self::Native => "Native",
			Self::Wasm => "Wasm",
			Self::Both => "Both",
			Self::NativeElseWasm => "NativeElseWasm",
		}
	}
}

arg_enum! {
	/// Available RPC methods.
	#[allow(missing_docs)]
	#[derive(Debug, Copy, Clone, PartialEq)]
	pub enum RpcMethods {
		// Expose every RPC method only when RPC is listening on `localhost`,
		// otherwise serve only safe RPC methods.
		Auto,
		// Allow only a safe subset of RPC methods.
		Safe,
		// Expose every RPC method (even potentially unsafe ones).
		Unsafe,
	}
}

impl Into<tc_service::config::RpcMethods> for RpcMethods {
	fn into(self) -> tc_service::config::RpcMethods {
		match self {
			RpcMethods::Auto => tc_service::config::RpcMethods::Auto,
			RpcMethods::Safe => tc_service::config::RpcMethods::Safe,
			RpcMethods::Unsafe => tc_service::config::RpcMethods::Unsafe,
		}
	}
}

arg_enum! {
	/// Database backend
	#[allow(missing_docs)]
	#[derive(Debug, Clone, Copy)]
	pub enum Database {
		// Facebooks RocksDB
		RocksDb,
		// TetsyDb. https://github.com/tetcoin/tetsy-db/
		TetsyDb,
	}
}


arg_enum! {
	/// Whether off-chain workers are enabled.
	#[allow(missing_docs)]
	#[derive(Debug, Clone)]
	pub enum OffchainWorkerEnabled {
		Always,
		Never,
		WhenValidating,
	}
}

/// Default value for the `--execution-syncing` parameter.
pub const DEFAULT_EXECUTION_SYNCING: ExecutionStrategy = ExecutionStrategy::NativeElseWasm;
/// Default value for the `--execution-import-block` parameter.
pub const DEFAULT_EXECUTION_IMPORT_BLOCK: ExecutionStrategy = ExecutionStrategy::NativeElseWasm;
/// Default value for the `--execution-import-block` parameter when the node is a validator.
pub const DEFAULT_EXECUTION_IMPORT_BLOCK_VALIDATOR: ExecutionStrategy = ExecutionStrategy::Wasm;
/// Default value for the `--execution-block-construction` parameter.
pub const DEFAULT_EXECUTION_BLOCK_CONSTRUCTION: ExecutionStrategy = ExecutionStrategy::Wasm;
/// Default value for the `--execution-offchain-worker` parameter.
pub const DEFAULT_EXECUTION_OFFCHAIN_WORKER: ExecutionStrategy = ExecutionStrategy::Native;
/// Default value for the `--execution-other` parameter.
pub const DEFAULT_EXECUTION_OTHER: ExecutionStrategy = ExecutionStrategy::Native;
