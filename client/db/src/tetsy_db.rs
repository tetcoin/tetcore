// This file is part of Tetcore.

// Copyright (C) 2017-2021 Parity Technologies (UK) Ltd.
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
/// A `Database` adapter for tetsy-db.

use tetcore_database::{Database, Change, ColumnId, Transaction, error::DatabaseError};
use crate::utils::{DatabaseType, NUM_COLUMNS};
use crate::columns;

struct DbAdapter(tetsy_db::Db);

fn handle_err<T>(result: tetsy_db::Result<T>) -> T {
	match result {
		Ok(r) => r,
		Err(e) =>  {
			panic!("Critical database error: {:?}", e);
		}
	}
}

/// Wrap tetsy-db database into a trait object that implements `tetcore_database::Database`
pub fn open<H: Clone>(path: &std::path::Path, db_type: DatabaseType)
	-> tetsy_db::Result<std::sync::Arc<dyn Database<H>>>
{
	let mut config = tetsy_db::Options::with_columns(path, NUM_COLUMNS as u8);
	config.sync = true; // Flush each commit
	if db_type == DatabaseType::Full {
		let mut state_col = &mut config.columns[columns::STATE as usize];
		state_col.ref_counted = true;
		state_col.preimage = true;
		state_col.uniform = true;
	}
	let db = tetsy_db::Db::open(&config)?;
	Ok(std::sync::Arc::new(DbAdapter(db)))
}

impl<H: Clone> Database<H> for DbAdapter {
	fn commit(&self, transaction: Transaction<H>) -> Result<(), DatabaseError> {
		handle_err(self.0.commit(transaction.0.into_iter().map(|change|
			match change {
				Change::Set(col, key, value) => (col as u8, key, Some(value)),
				Change::Remove(col, key) => (col as u8, key, None),
				_ => unimplemented!(),
			}))
		);

		Ok(())
	}

	fn get(&self, col: ColumnId, key: &[u8]) -> Option<Vec<u8>> {
		handle_err(self.0.get(col as u8, key))
	}

	fn lookup(&self, _hash: &H) -> Option<Vec<u8>> {
		unimplemented!();
	}
}
