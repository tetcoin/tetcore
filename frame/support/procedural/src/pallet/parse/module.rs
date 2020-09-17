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
use quote::ToTokens;

/// List of additional token to be used for parsing.
mod keyword {
	syn::custom_keyword!(pallet);
	syn::custom_keyword!(Module);
	syn::custom_keyword!(generate_store);
	syn::custom_keyword!(Store);
}

/// Definition of the pallet module.
pub struct ModuleDef {
	/// The index of error item in pallet module.
	pub index: usize,
	/// A set of usage of instance, must be check for consistency with trait.
	pub instances: Vec<helper::InstanceUsage>,
	/// The keyword module used (contains span).
	pub module: keyword::Module,
	/// Weither the trait `Store` must be generated.
	pub store: Option<(syn::Visibility, keyword::Store)>
}

/// Parse for `#[pallet::generate($vis fn deposit_event)]`
pub struct PalletModuleAttr {
	vis: syn::Visibility,
	keyword: keyword::Store,
}

impl syn::parse::Parse for PalletModuleAttr {
	fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
		input.parse::<syn::Token![#]>()?;
		let content;
		syn::bracketed!(content in input);
		content.parse::<keyword::pallet>()?;
		content.parse::<syn::Token![::]>()?;
		content.parse::<keyword::generate_store>()?;

		let generate_content;
		syn::parenthesized!(generate_content in content);
		let vis = generate_content.parse::<syn::Visibility>()?;
		generate_content.parse::<syn::Token![trait]>()?;
		let keyword = generate_content.parse::<keyword::Store>()?;
		Ok(Self { vis, keyword })
	}
}

impl ModuleDef {
	pub fn try_from(index: usize, item: &mut syn::Item) -> syn::Result<Self> {
		let item = if let syn::Item::Struct(item) = item {
			item
		} else {
			let msg = "Invalid pallet::module, expect struct definition";
			return Err(syn::Error::new(item.span(), msg));
		};

		let mut event_attrs: Vec<PalletModuleAttr> = helper::take_item_attrs(&mut item.attrs)?;
		if event_attrs.len() > 1 {
			let msg = "Invalid pallet::module, multiple argument pallet::generate_store found";
			return Err(syn::Error::new(event_attrs[1].keyword.span(), msg));
		}
		let store = event_attrs.pop().map(|attr| (attr.vis, attr.keyword));

		let module = syn::parse2::<keyword::Module>(item.ident.to_token_stream())?;

		if !matches!(item.vis, syn::Visibility::Public(_)) {
			let msg = "Invalid pallet::module, Module must be public";
			return Err(syn::Error::new(item.span(), msg));
		}
		if item.generics.where_clause.is_some() {
			let msg = "Invalid pallet::module, where clause not supported on Module declaration";
			return Err(syn::Error::new(item.generics.where_clause.span(), msg));
		}

		let mut instances = vec![];
		instances.push(helper::check_type_def_generics(&item.generics, item.ident.span())?);

		Ok(Self { index, instances, module, store })
	}
}
