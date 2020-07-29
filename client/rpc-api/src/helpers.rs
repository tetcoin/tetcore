// This file is part of Substrate.

// Copyright (C) 2018-2020 Parity Technologies (UK) Ltd.
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

use std::pin::Pin;
use jsonrpc_core::Error;
use futures::{Future, FutureExt, channel::oneshot, task::{Poll, Context}};

/// Wraps around `oneshot::Receiver` and adjusts the error type to produce an internal error if the
/// sender gets dropped.
pub struct Receiver<T>(pub oneshot::Receiver<T>);

impl<T> Future for Receiver<T> {
	type Output = Result<T, Error>;

	fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<T, Error>> {
		self.0.poll_unpin(cx).map_err(|_| Error::internal_error())
	}
}
