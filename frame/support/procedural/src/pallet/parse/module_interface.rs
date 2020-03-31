// This file is part of Substrate.

// Copyright (C) 2020 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use syn::spanned::Spanned;
use super::helper;

pub struct ModuleInterfaceDef {
	/// The index of error item in pallet module.
	pub index: usize,
	/// A set of usage of instance, must be check for consistency with trait.
	pub instances: Vec<helper::InstanceUsage>,
}

// TODO TODO: ensure blocknumber is frame_system blocknumber or write it in the doc
impl ModuleInterfaceDef {
	pub fn try_from(index: usize, item: &mut syn::Item) -> syn::Result<Self> {
		let item = if let syn::Item::Impl(item) = item {
			item
		} else {
			let msg = "Invalid pallet::module_interface, expect item impl";
			return Err(syn::Error::new(item.span(), msg));
		};

		let item_trait = &item.trait_.as_ref()
			.ok_or_else(|| {
				let msg = "Invalid pallet::module_interface, expect impl... ModuleInterface \
					for ...";
				syn::Error::new(item.span(), msg)
			})?.1;

		if item_trait.segments.len() != 1
			|| item_trait.segments[0].ident != "ModuleInterface"
		{
			let msg = "Invalid pallet::module_interface, expect trait to be `ModuleInterface`";
			return Err(syn::Error::new(item_trait.span(), msg));
		}

		let mut instances = vec![];
		instances.push(helper::check_module_usage(&item.self_ty)?);

		Ok(Self { index, instances })
	}
}

