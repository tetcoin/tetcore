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

use super::helper;
use syn::spanned::Spanned;

/// List of additional token to be used for parsing.
mod keyword {
	syn::custom_keyword!(Error);
}

pub enum Metadata{
	Value { value: syn::GenericArgument },
	Map { value: syn::GenericArgument, key: syn::GenericArgument },
	DoubleMap {
		value: syn::GenericArgument,
		key1: syn::GenericArgument,
		key2: syn::GenericArgument
	},
}

/// TODO TODO: doc
pub struct StorageDef {
	/// The index of error item in pallet module.
	pub index: usize,
	pub vis: syn::Visibility,
	/// The type ident, to generate the StoragePrefix for.
	pub ident: syn::Ident,
	/// If event is declared with instance.
	pub has_instance: bool,
	pub has_trait: bool,
	pub metadata: Metadata,
	pub docs: Vec<syn::Lit>,
	/// A set of usage of instance, must be check for consistency with trait.
	pub instances: Vec<helper::InstanceUsage>,
}

fn retrieve_arg(
	segment: &syn::PathSegment,
	arg_pos: usize,
) -> syn::Result<syn::GenericArgument> {
	if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
		if arg_pos < args.args.len() {
			Ok(args.args[arg_pos].clone())
		} else {
			let msg = format!("pallet::storage unexpected number of generic argument, expect at \
				least {} args, found {}", arg_pos + 1, args.args.len());
			Err(syn::Error::new(args.span(), msg))
		}
	} else {
		let msg = format!("pallet::storage unexpected number of generic argument, expect at least \
			{} args, found none", arg_pos + 1);
		Err(syn::Error::new(segment.span(), msg))
	}
}

impl StorageDef {
	pub fn try_from(index: usize, item: &mut syn::Item) -> syn::Result<Self> {
		let item = if let syn::Item::Type(item) = item {
			item
		} else {
			return Err(syn::Error::new(item.span(), "Invalid pallet::error, expect item enum"));
		};

		let mut instances = vec![];
		instances.push(helper::check_storage_optional_gen(&item.generics, item.span())?);

		if item.generics.where_clause.is_some() {
			let msg = "Invalid pallet::storage, unexpected where clause";
			return Err(syn::Error::new(item.generics.where_clause.as_ref().unwrap().span(), msg));
		}

		let docs = helper::get_doc_literals(&item.attrs);

		let typ = if let syn::Type::Path(typ) = &*item.ty {
			typ
		} else {
			let msg = "Invalid pallet::storage, expect type path";
			return Err(syn::Error::new(item.ty.span(), msg));
		};

		if typ.path.segments.len() != 1 {
			let msg = "Invalid pallet::storage, expect type path with one segment";
			return Err(syn::Error::new(item.ty.span(), msg));
		}

		let metadata = match &*typ.path.segments[0].ident.to_string() {
			"StorageValueType" => {
				Metadata::Value {
					value: retrieve_arg(&typ.path.segments[0], 1)?,
				}
			}
			"StorageMapType" => {
				Metadata::Map {
					key:  retrieve_arg(&typ.path.segments[0], 1)?,
					value:  retrieve_arg(&typ.path.segments[0], 2)?,
				}
			}
			"StorageDoubleMapType" => {
				Metadata::DoubleMap {
					key1:  retrieve_arg(&typ.path.segments[0], 1)?,
					key2:  retrieve_arg(&typ.path.segments[0], 2)?, // TODO TODO: maybe use stringify instead of debug format
					value:  retrieve_arg(&typ.path.segments[0], 3)?,
				}
			}
			_ => {
				let msg = "Invalid pallet::storage, expect ident: `StorageValueType` or \
				`StorageMapType` or `StorageDoubleMapType` in order to expand metadata"; // TODO TODO: add found
				return Err(syn::Error::new(item.ty.span(), msg));
			}
		};

		let has_instance = item.generics.type_params().any(|t| t.ident == "I");
		let has_trait = item.generics.type_params().any(|t| t.ident == "T");

		Ok(StorageDef {
			index,
			vis: item.vis.clone(),
			ident: item.ident.clone(),
			has_instance,
			has_trait,
			instances,
			metadata,
			docs,
		})
	}
}
