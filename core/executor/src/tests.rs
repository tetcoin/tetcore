use state_machine::TestExternalities as CoreTestExternalities;
use hex_literal::hex;
use primitives::{
	Blake2Hasher, blake2_128, blake2_256, ed25519, sr25519, map, Pair, offchain::OffchainExt,
	traits::Externalities,
};
use runtime_test::WASM_BINARY;
use substrate_offchain::testing;
use trie::{TrieConfiguration, trie_types::Layout};
use codec::{Encode, Decode};
use test_case::test_case;

use crate::{WasmExecutionMethod, call_in_wasm};
use crate::error::Error;

type TestExternalities = CoreTestExternalities<Blake2Hasher, u64>;

#[test_case(WasmExecutionMethod::Interpreted)]
fn returning_should_work(wasm_method: WasmExecutionMethod) {
	let mut ext = TestExternalities::default();
	let mut ext = ext.ext();
	let test_code = WASM_BINARY;

	let output = call_in_wasm(
		"test_empty_return",
		&[],
		wasm_method,
		&mut ext,
		&test_code[..],
		8,
	).unwrap();
	assert_eq!(output, vec![0u8; 0]);
}

#[test_case(WasmExecutionMethod::Interpreted)]
fn panicking_should_work(wasm_method: WasmExecutionMethod) {
	let mut ext = TestExternalities::default();
	let mut ext = ext.ext();
	let test_code = WASM_BINARY;

	let output = call_in_wasm(
		"test_panic",
		&[],
		wasm_method,
		&mut ext,
		&test_code[..],
		8,
	);
	assert!(output.is_err());

	let output = call_in_wasm(
		"test_conditional_panic",
		&[0],
		wasm_method,
		&mut ext,
		&test_code[..],
		8,
	);
	assert_eq!(Decode::decode(&mut &output.unwrap()[..]), Ok(Vec::<u8>::new()));

	let output = call_in_wasm(
		"test_conditional_panic",
		&vec![2].encode(),
		wasm_method,
		&mut ext,
		&test_code[..],
		8,
	);
	assert!(output.is_err());
}

#[test_case(WasmExecutionMethod::Interpreted)]
fn storage_should_work(wasm_method: WasmExecutionMethod) {
	let mut ext = TestExternalities::default();

	{
		let mut ext = ext.ext();
		ext.set_storage(b"foo".to_vec(), b"bar".to_vec());
		let test_code = WASM_BINARY;

		let output = call_in_wasm(
			"test_data_in",
			&b"Hello world".to_vec().encode(),
			wasm_method,
			&mut ext,
			&test_code[..],
			8,
		).unwrap();

		assert_eq!(output, b"all ok!".to_vec().encode());
	}

	let expected = TestExternalities::new((map![
			b"input".to_vec() => b"Hello world".to_vec(),
			b"foo".to_vec() => b"bar".to_vec(),
			b"baz".to_vec() => b"bar".to_vec()
		], map![]));
	assert_eq!(ext, expected);
}

#[test_case(WasmExecutionMethod::Interpreted)]
fn clear_prefix_should_work(wasm_method: WasmExecutionMethod) {
	let mut ext = TestExternalities::default();
	{
		let mut ext = ext.ext();
		ext.set_storage(b"aaa".to_vec(), b"1".to_vec());
		ext.set_storage(b"aab".to_vec(), b"2".to_vec());
		ext.set_storage(b"aba".to_vec(), b"3".to_vec());
		ext.set_storage(b"abb".to_vec(), b"4".to_vec());
		ext.set_storage(b"bbb".to_vec(), b"5".to_vec());
		let test_code = WASM_BINARY;

		// This will clear all entries which prefix is "ab".
		let output = call_in_wasm(
			"test_clear_prefix",
			&b"ab".to_vec().encode(),
			wasm_method,
			&mut ext,
			&test_code[..],
			8,
		).unwrap();

		assert_eq!(output, b"all ok!".to_vec().encode());
	}

	let expected = TestExternalities::new((map![
			b"aaa".to_vec() => b"1".to_vec(),
			b"aab".to_vec() => b"2".to_vec(),
			b"bbb".to_vec() => b"5".to_vec()
		], map![]));
	assert_eq!(expected, ext);
}

#[test_case(WasmExecutionMethod::Interpreted)]
fn blake2_256_should_work(wasm_method: WasmExecutionMethod) {
	let mut ext = TestExternalities::default();
	let mut ext = ext.ext();
	let test_code = WASM_BINARY;
	assert_eq!(
		call_in_wasm(
			"test_blake2_256",
			&[0],
			wasm_method,
			&mut ext,
			&test_code[..],
			8,
		).unwrap(),
		blake2_256(&b""[..]).to_vec().encode(),
	);
	assert_eq!(
		call_in_wasm(
			"test_blake2_256",
			&b"Hello world!".to_vec().encode(),
			wasm_method,
			&mut ext,
			&test_code[..],
			8,
		).unwrap(),
		blake2_256(&b"Hello world!"[..]).to_vec().encode(),
	);
}

#[test_case(WasmExecutionMethod::Interpreted)]
fn blake2_128_should_work(wasm_method: WasmExecutionMethod) {
	let mut ext = TestExternalities::default();
	let mut ext = ext.ext();
	let test_code = WASM_BINARY;
	assert_eq!(
		call_in_wasm(
			"test_blake2_128",
			&[0],
			wasm_method,
			&mut ext,
			&test_code[..],
			8,
		).unwrap(),
		blake2_128(&b""[..]).to_vec().encode(),
	);
	assert_eq!(
		call_in_wasm(
			"test_blake2_128",
			&b"Hello world!".to_vec().encode(),
			wasm_method,
			&mut ext,
			&test_code[..],
			8,
		).unwrap(),
		blake2_128(&b"Hello world!"[..]).to_vec().encode(),
	);
}

#[test_case(WasmExecutionMethod::Interpreted)]
fn twox_256_should_work(wasm_method: WasmExecutionMethod) {
	let mut ext = TestExternalities::default();
	let mut ext = ext.ext();
	let test_code = WASM_BINARY;
	assert_eq!(
		call_in_wasm(
			"test_twox_256",
			&[0],
			wasm_method,
			&mut ext,
			&test_code[..],
			8,
		).unwrap(),
		hex!(
				"99e9d85137db46ef4bbea33613baafd56f963c64b1f3685a4eb4abd67ff6203a"
			).to_vec().encode(),
	);
	assert_eq!(
		call_in_wasm(
			"test_twox_256",
			&b"Hello world!".to_vec().encode(),
			wasm_method,
			&mut ext,
			&test_code[..],
			8,
		).unwrap(),
		hex!(
				"b27dfd7f223f177f2a13647b533599af0c07f68bda23d96d059da2b451a35a74"
			).to_vec().encode(),
	);
}

#[test_case(WasmExecutionMethod::Interpreted)]
fn twox_128_should_work(wasm_method: WasmExecutionMethod) {
	let mut ext = TestExternalities::default();
	let mut ext = ext.ext();
	let test_code = WASM_BINARY;
	assert_eq!(
		call_in_wasm(
			"test_twox_128",
			&[0],
			wasm_method,
			&mut ext,
			&test_code[..],
			8,
		).unwrap(),
		hex!("99e9d85137db46ef4bbea33613baafd5").to_vec().encode(),
	);
	assert_eq!(
		call_in_wasm(
			"test_twox_128",
			&b"Hello world!".to_vec().encode(),
			wasm_method,
			&mut ext,
			&test_code[..],
			8,
		).unwrap(),
		hex!("b27dfd7f223f177f2a13647b533599af").to_vec().encode(),
	);
}

#[test_case(WasmExecutionMethod::Interpreted)]
fn ed25519_verify_should_work(wasm_method: WasmExecutionMethod) {
	let mut ext = TestExternalities::default();
	let mut ext = ext.ext();
	let test_code = WASM_BINARY;
	let key = ed25519::Pair::from_seed(&blake2_256(b"test"));
	let sig = key.sign(b"all ok!");
	let mut calldata = vec![];
	calldata.extend_from_slice(key.public().as_ref());
	calldata.extend_from_slice(sig.as_ref());

	assert_eq!(
		call_in_wasm(
			"test_ed25519_verify",
			&calldata.encode(),
			wasm_method,
			&mut ext,
			&test_code[..],
			8,
		).unwrap(),
		true.encode(),
	);

	let other_sig = key.sign(b"all is not ok!");
	let mut calldata = vec![];
	calldata.extend_from_slice(key.public().as_ref());
	calldata.extend_from_slice(other_sig.as_ref());

	assert_eq!(
		call_in_wasm(
			"test_ed25519_verify",
			&calldata.encode(),
			wasm_method,
			&mut ext,
			&test_code[..],
			8,
		).unwrap(),
		false.encode(),
	);
}

#[test_case(WasmExecutionMethod::Interpreted)]
fn sr25519_verify_should_work(wasm_method: WasmExecutionMethod) {
	let mut ext = TestExternalities::default();
	let mut ext = ext.ext();
	let test_code = WASM_BINARY;
	let key = sr25519::Pair::from_seed(&blake2_256(b"test"));
	let sig = key.sign(b"all ok!");
	let mut calldata = vec![];
	calldata.extend_from_slice(key.public().as_ref());
	calldata.extend_from_slice(sig.as_ref());

	assert_eq!(
		call_in_wasm(
			"test_sr25519_verify",
			&calldata.encode(),
			wasm_method,
			&mut ext,
			&test_code[..],
			8,
		).unwrap(),
		true.encode(),
	);

	let other_sig = key.sign(b"all is not ok!");
	let mut calldata = vec![];
	calldata.extend_from_slice(key.public().as_ref());
	calldata.extend_from_slice(other_sig.as_ref());

	assert_eq!(
		call_in_wasm(
			"test_sr25519_verify",
			&calldata.encode(),
			wasm_method,
			&mut ext,
			&test_code[..],
			8,
		).unwrap(),
		false.encode(),
	);
}

#[test_case(WasmExecutionMethod::Interpreted)]
fn ordered_trie_root_should_work(wasm_method: WasmExecutionMethod) {
	let mut ext = TestExternalities::default();
	let mut ext = ext.ext();
	let trie_input = vec![b"zero".to_vec(), b"one".to_vec(), b"two".to_vec()];
	let test_code = WASM_BINARY;
	assert_eq!(
		call_in_wasm(
			"test_ordered_trie_root",
			&[0],
			wasm_method,
			&mut ext,
			&test_code[..],
			8,
		).unwrap(),
		Layout::<Blake2Hasher>::ordered_trie_root(trie_input.iter()).as_bytes().encode(),
	);
}

#[test_case(WasmExecutionMethod::Interpreted)]
fn offchain_local_storage_should_work(wasm_method: WasmExecutionMethod) {
	use substrate_client::backend::OffchainStorage;

	let mut ext = TestExternalities::default();
	let (offchain, state) = testing::TestOffchainExt::new();
	ext.register_extension(OffchainExt::new(offchain));
	let test_code = WASM_BINARY;
	let mut ext = ext.ext();
	assert_eq!(
		call_in_wasm(
			"test_offchain_local_storage",
			&[0],
			wasm_method,
			&mut ext,
			&test_code[..],
			8,
		).unwrap(),
		true.encode(),
	);
	assert_eq!(state.read().persistent_storage.get(b"", b"test"), Some(vec![]));
}

#[test_case(WasmExecutionMethod::Interpreted)]
fn offchain_http_should_work(wasm_method: WasmExecutionMethod) {
	let mut ext = TestExternalities::default();
	let (offchain, state) = testing::TestOffchainExt::new();
	ext.register_extension(OffchainExt::new(offchain));
	state.write().expect_request(
		0,
		testing::PendingRequest {
			method: "POST".into(),
			uri: "http://localhost:12345".into(),
			body: vec![1, 2, 3, 4],
			headers: vec![("X-Auth".to_owned(), "test".to_owned())],
			sent: true,
			response: Some(vec![1, 2, 3]),
			response_headers: vec![("X-Auth".to_owned(), "hello".to_owned())],
			..Default::default()
		},
	);

	let test_code = WASM_BINARY;
	let mut ext = ext.ext();
	assert_eq!(
		call_in_wasm(
			"test_offchain_http",
			&[0],
			wasm_method,
			&mut ext,
			&test_code[..],
			8,
		).unwrap(),
		true.encode(),
	);
}

mod sandbox {
	use super::*;
	use test_case::test_case;
	use wabt;

	#[test_case(WasmExecutionMethod::Interpreted)]
	fn sandbox_should_work(wasm_method: WasmExecutionMethod) {
		let mut ext = TestExternalities::default();
		let mut ext = ext.ext();
		let test_code = WASM_BINARY;

		let code = wabt::wat2wasm(r#"
		(module
			(import "env" "assert" (func $assert (param i32)))
			(import "env" "inc_counter" (func $inc_counter (param i32) (result i32)))
			(func (export "call")
				(drop
					(call $inc_counter (i32.const 5))
				)

				(call $inc_counter (i32.const 3))
				;; current counter value is on the stack

				;; check whether current == 8
				i32.const 8
				i32.eq

				call $assert
			)
		)
		"#).unwrap().encode();

		assert_eq!(
			call_in_wasm(
				"test_sandbox",
				&code,
				wasm_method,
				&mut ext,
				&test_code[..],
				8,
			).unwrap(),
			true.encode(),
		);
	}

	#[test_case(WasmExecutionMethod::Interpreted)]
	fn sandbox_trap(wasm_method: WasmExecutionMethod) {
		let mut ext = TestExternalities::default();
		let mut ext = ext.ext();
		let test_code = WASM_BINARY;

		let code = wabt::wat2wasm(r#"
		(module
			(import "env" "assert" (func $assert (param i32)))
			(func (export "call")
				i32.const 0
				call $assert
			)
		)
		"#).unwrap();

		assert_eq!(
			call_in_wasm(
				"test_sandbox",
				&code,
				wasm_method,
				&mut ext,
				&test_code[..],
				8,
			).unwrap(),
			vec![0],
		);
	}

	#[test_case(WasmExecutionMethod::Interpreted)]
	fn sandbox_should_trap_when_heap_exhausted(wasm_method: WasmExecutionMethod) {
		let mut ext = TestExternalities::default();
		let mut ext = ext.ext();
		let test_code = WASM_BINARY;

		let code = wabt::wat2wasm(r#"
		(module
			(import "env" "assert" (func $assert (param i32)))
			(func (export "call")
				i32.const 0
				call $assert
			)
		)
		"#).unwrap().encode();

		let res = call_in_wasm(
			"test_exhaust_heap",
			&code,
			wasm_method,
			&mut ext,
			&test_code[..],
			8,
		);
		assert_eq!(res.is_err(), true);
		if let Err(err) = res {
			assert_eq!(
				format!("{}", err),
				format!(
					"{}",
					wasmi::Error::Trap(Error::FunctionExecution("AllocatorOutOfSpace".into()).into()),
				),
			);
		}
	}

	#[test_case(WasmExecutionMethod::Interpreted)]
	fn start_called(wasm_method: WasmExecutionMethod) {
		let mut ext = TestExternalities::default();
		let mut ext = ext.ext();
		let test_code = WASM_BINARY;

		let code = wabt::wat2wasm(r#"
		(module
			(import "env" "assert" (func $assert (param i32)))
			(import "env" "inc_counter" (func $inc_counter (param i32) (result i32)))

			;; Start function
			(start $start)
			(func $start
				;; Increment counter by 1
				(drop
					(call $inc_counter (i32.const 1))
				)
			)

			(func (export "call")
				;; Increment counter by 1. The current value is placed on the stack.
				(call $inc_counter (i32.const 1))

				;; Counter is incremented twice by 1, once there and once in `start` func.
				;; So check the returned value is equal to 2.
				i32.const 2
				i32.eq
				call $assert
			)
		)
		"#).unwrap().encode();

		assert_eq!(
			call_in_wasm(
				"test_sandbox",
				&code,
				wasm_method,
				&mut ext,
				&test_code[..],
				8,
			).unwrap(),
			true.encode(),
		);
	}

	#[test_case(WasmExecutionMethod::Interpreted)]
	fn invoke_args(wasm_method: WasmExecutionMethod) {
		let mut ext = TestExternalities::default();
		let mut ext = ext.ext();
		let test_code = WASM_BINARY;

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
		"#).unwrap().encode();

		assert_eq!(
			call_in_wasm(
				"test_sandbox_args",
				&code,
				wasm_method,
				&mut ext,
				&test_code[..],
				8,
			).unwrap(),
			true.encode(),
		);
	}

	#[test_case(WasmExecutionMethod::Interpreted)]
	fn return_val(wasm_method: WasmExecutionMethod) {
		let mut ext = TestExternalities::default();
		let mut ext = ext.ext();
		let test_code = WASM_BINARY;

		let code = wabt::wat2wasm(r#"
		(module
			(func (export "call") (param $x i32) (result i32)
				(i32.add
					(get_local $x)
					(i32.const 1)
				)
			)
		)
		"#).unwrap().encode();

		assert_eq!(
			call_in_wasm(
				"test_sandbox_return_val",
				&code,
				wasm_method,
				&mut ext,
				&test_code[..],
				8,
			).unwrap(),
			true.encode(),
		);
	}

	#[test_case(WasmExecutionMethod::Interpreted)]
	fn unlinkable_module(wasm_method: WasmExecutionMethod) {
		let mut ext = TestExternalities::default();
		let mut ext = ext.ext();
		let test_code = WASM_BINARY;

		let code = wabt::wat2wasm(r#"
		(module
			(import "env" "non-existent" (func))

			(func (export "call")
			)
		)
		"#).unwrap().encode();

		assert_eq!(
			call_in_wasm(
				"test_sandbox_instantiate",
				&code,
				wasm_method,
				&mut ext,
				&test_code[..],
				8,
			).unwrap(),
			1u8.encode(),
		);
	}

	#[test_case(WasmExecutionMethod::Interpreted)]
	fn corrupted_module(wasm_method: WasmExecutionMethod) {
		let mut ext = TestExternalities::default();
		let mut ext = ext.ext();
		let test_code = WASM_BINARY;

		// Corrupted wasm file
		let code = vec![0u8, 0, 0, 0, 1, 0, 0, 0].encode();

		assert_eq!(
			call_in_wasm(
				"test_sandbox_instantiate",
				&code,
				wasm_method,
				&mut ext,
				&test_code[..],
				8,
			).unwrap(),
			1u8.encode(),
		);
	}

	#[test_case(WasmExecutionMethod::Interpreted)]
	fn start_fn_ok(wasm_method: WasmExecutionMethod) {
		let mut ext = TestExternalities::default();
		let mut ext = ext.ext();
		let test_code = WASM_BINARY;

		let code = wabt::wat2wasm(r#"
		(module
			(func (export "call")
			)

			(func $start
			)

			(start $start)
		)
		"#).unwrap().encode();

		assert_eq!(
			call_in_wasm(
				"test_sandbox_instantiate",
				&code,
				wasm_method,
				&mut ext,
				&test_code[..],
				8,
			).unwrap(),
			0u8.encode(),
		);
	}

	#[test_case(WasmExecutionMethod::Interpreted)]
	fn start_fn_traps(wasm_method: WasmExecutionMethod) {
		let mut ext = TestExternalities::default();
		let mut ext = ext.ext();
		let test_code = WASM_BINARY;

		let code = wabt::wat2wasm(r#"
		(module
			(func (export "call")
			)

			(func $start
				unreachable
			)

			(start $start)
		)
		"#).unwrap().encode();

		assert_eq!(
			call_in_wasm(
				"test_sandbox_instantiate",
				&code,
				wasm_method,
				&mut ext,
				&test_code[..],
				8,
			).unwrap(),
			2u8.encode(),
		);
	}
}
