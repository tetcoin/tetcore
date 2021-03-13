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

use structopt::StructOpt;
use tc_cli::{
	Error, VanityCmd, SignCmd, VerifyCmd, GenerateNodeKeyCmd, GenerateCmd, InspectKeyCmd,
	InspectNodeKeyCmd
};

#[derive(Debug, StructOpt)]
#[structopt(
	name = "tetkey",
	author = "Parity Team <admin@parity.io>",
	about = "Utility for generating and restoring with Tetcore keys",
)]
pub enum Tetkey {
	/// Generate a random node libp2p key, save it to file or print it to stdout
	/// and print its peer ID to stderr.
	GenerateNodeKey(GenerateNodeKeyCmd),

	/// Generate a random account
	Generate(GenerateCmd),

	/// Gets a public key and a SS58 address from the provided Secret URI
	Inspect(InspectKeyCmd),

	/// Print the peer ID corresponding to the node key in the given file
	InspectNodeKey(InspectNodeKeyCmd),

	/// Sign a message, with a given (secret) key.
	Sign(SignCmd),

	/// Generate a seed that provides a vanity address.
	Vanity(VanityCmd),

	/// Verify a signature for a message, provided on STDIN, with a given (public or secret) key.
	Verify(VerifyCmd),
}

/// Run the tetkey command, given the apropriate runtime.
pub fn run() -> Result<(), Error> {
	match Tetkey::from_args() {
		Tetkey::GenerateNodeKey(cmd) => cmd.run(),
		Tetkey::Generate(cmd) => cmd.run(),
		Tetkey::Inspect(cmd) => cmd.run(),
		Tetkey::InspectNodeKey(cmd) => cmd.run(),
		Tetkey::Vanity(cmd) => cmd.run(),
		Tetkey::Verify(cmd) => cmd.run(),
		Tetkey::Sign(cmd) => cmd.run(),
	}
}
