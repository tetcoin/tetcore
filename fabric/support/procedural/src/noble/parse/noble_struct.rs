// This file is part of Tetcore.

// Copyright (C) 2020-2021 Parity Technologies (UK) Ltd.
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
	syn::custom_keyword!(noble);
	syn::custom_keyword!(Noble);
	syn::custom_keyword!(generate_store);
	syn::custom_keyword!(Store);
}

/// Definition of the noble noble.
pub struct NobleStructDef {
	/// The index of item in noble noble.
	pub index: usize,
	/// A set of usage of instance, must be check for consistency with config trait.
	pub instances: Vec<helper::InstanceUsage>,
	/// The keyword Noble used (contains span).
	pub noble: keyword::Noble,
	/// Whether the trait `Store` must be generated.
	pub store: Option<(syn::Visibility, keyword::Store)>,
	/// The span of the noble::noble attribute.
	pub attr_span: proc_macro2::Span,
}

/// Parse for `#[noble::generate_store($vis trait Store)]`
pub struct NobleStructAttr {
	vis: syn::Visibility,
	keyword: keyword::Store,
}

impl syn::parse::Parse for NobleStructAttr {
	fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
		input.parse::<syn::Token![#]>()?;
		let content;
		syn::bracketed!(content in input);
		content.parse::<keyword::noble>()?;
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

impl NobleStructDef {
	pub fn try_from(
		attr_span: proc_macro2::Span,
		index: usize,
		item: &mut syn::Item,
	) -> syn::Result<Self> {
		let item = if let syn::Item::Struct(item) = item {
			item
		} else {
			let msg = "Invalid noble::noble, expected struct definition";
			return Err(syn::Error::new(item.span(), msg));
		};

		let mut event_attrs: Vec<NobleStructAttr> = helper::take_item_attrs(&mut item.attrs)?;
		if event_attrs.len() > 1 {
			let msg = "Invalid noble::noble, multiple argument noble::generate_store found";
			return Err(syn::Error::new(event_attrs[1].keyword.span(), msg));
		}
		let store = event_attrs.pop().map(|attr| (attr.vis, attr.keyword));

		let noble = syn::parse2::<keyword::Noble>(item.ident.to_token_stream())?;

		if !matches!(item.vis, syn::Visibility::Public(_)) {
			let msg = "Invalid noble::noble, Noble must be public";
			return Err(syn::Error::new(item.span(), msg));
		}

		if item.generics.where_clause.is_some() {
			let msg = "Invalid noble::noble, where clause not supported on Noble declaration";
			return Err(syn::Error::new(item.generics.where_clause.span(), msg));
		}

		let mut instances = vec![];
		instances.push(helper::check_type_def_gen_no_bounds(&item.generics, item.ident.span())?);

		Ok(Self { index, instances, noble, store, attr_span })
	}
}
