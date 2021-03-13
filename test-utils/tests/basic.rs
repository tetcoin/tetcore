// This file is part of Tetcore.

// Copyright (C) 2020-2021 Parity Technologies (UK) Ltd.
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

use tc_service::{TaskExecutor, TaskType};

#[tetcore_test_utils::test]
async fn basic_test(_: TaskExecutor) {
	assert!(true);
}

#[tetcore_test_utils::test]
#[should_panic(expected = "boo!")]
async fn panicking_test(_: TaskExecutor) {
	panic!("boo!");
}

#[tetcore_test_utils::test(max_threads = 2)]
async fn basic_test_with_args(_: TaskExecutor) {
	assert!(true);
}

#[tetcore_test_utils::test]
async fn rename_argument(ex: TaskExecutor) {
	let ex2 = ex.clone();
	ex2.spawn(Box::pin(async { () }), TaskType::Blocking);
	assert!(true);
}

#[tetcore_test_utils::test]
#[should_panic(expected = "test took too long")]
// NOTE: enable this test only after setting TETCORE_TEST_TIMEOUT to a smaller value
//
// TETCORE_TEST_TIMEOUT=1 cargo test -- --ignored timeout
#[ignore]
async fn timeout(_: TaskExecutor) {
	tokio::time::delay_for(std::time::Duration::from_secs(
		std::env::var("TETCORE_TEST_TIMEOUT")
			.expect("env var TETCORE_TEST_TIMEOUT has been provided by the user")
			.parse::<u64>()
			.unwrap() + 1,
	))
	.await;
}
