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
	syn::custom_keyword!(Trait);
	syn::custom_keyword!(Get);
	syn::custom_keyword!(trait_);
	syn::custom_keyword!(const_);
}

pub struct TraitDef {
	/// The index of error item in pallet module.
	pub index: usize,
	/// Wheither the trait has instance (i.e. define with `Trait<I: Instance = DefaultInstance>`)
	pub has_instance: bool,
	/// Const associated type.
	pub consts_metadata: Vec<ConstMetadataDef>,
}

pub struct ConstMetadataDef {
	/// Name of the associated type.
	pub ident: syn::Ident,
	/// The type in Get, e.g. `u32` in `type Foo: Get<u32>;`
	pub type_: syn::Type,
	/// The doc associated
	pub doc: Vec<syn::Lit>,
}

impl syn::parse::Parse for ConstMetadataDef  {
	fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
		let doc = helper::get_doc_literals(&syn::Attribute::parse_outer(input)?);
		input.parse::<syn::Token![type]>()?;
		let ident = input.parse::<syn::Ident>()?;
		input.parse::<syn::Token![:]>()?;
		input.parse::<keyword::Get>()?;
		input.parse::<syn::Token![<]>()?;
		let type_ = input.parse::<syn::Type>()?;
		input.parse::<syn::Token![>]>()?;
		input.parse::<syn::Token![;]>()?;

		Ok(Self { ident, type_, doc })
	}
}

/// Parse for `#[pallet::const_]`
pub struct TypeAttrConst(proc_macro2::Span);

impl Spanned for TypeAttrConst {
	fn span(&self) -> proc_macro2::Span {
		self.0
	}
}

impl syn::parse::Parse for TypeAttrConst {
	fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
		input.parse::<syn::Token![#]>()?;
		let content;
		syn::bracketed!(content in input);
		content.parse::<syn::Ident>()?;
		content.parse::<syn::Token![::]>()?;

		Ok(TypeAttrConst(content.parse::<keyword::const_>()?.span()))
	}
}

impl TraitDef {
	pub fn try_from(index: usize, item: &mut syn::Item) -> syn::Result<Self> {
		let item = if let syn::Item::Trait(item) = item {
			item
		} else {
			let msg = "Invalid pallet::trait, expect Trait definition";
			return Err(syn::Error::new(item.span(), msg));
		};

		if !matches!(item.vis, syn::Visibility::Public(_)) {
			let msg = "Invalid pallet::trait_, Trait must be public";
			return Err(syn::Error::new(item.span(), msg));
		}

		syn::parse2::<keyword::Trait>(item.ident.to_token_stream())?;

		if item.generics.where_clause.is_some() {
			let msg = "Invalid pallet::trait, expect no where clause";
			return Err(syn::Error::new(item.generics.where_clause.span(), msg));
		}

		if item.generics.params.len() > 1 {
			let msg = "Invalid pallet::trait, expect no more than one generics";
			return Err(syn::Error::new(item.generics.params[2].span(), msg));
		}

		let has_instance;
		if let Some(_) = item.generics.params.first() {
			helper::check_trait_def_generics(&item.generics, item.ident.span())?;
			has_instance = true;
		} else {
			has_instance = false;
		}

		let mut consts_metadata = vec![];
		for trait_item in &mut item.items {
			let type_attrs_const: Vec<TypeAttrConst> = helper::take_item_attrs(trait_item)?;

			if type_attrs_const.len() > 1 {
				let msg = "Invalid attribute in pallet::trait, only one attribute is expected";
				return Err(syn::Error::new(type_attrs_const[1].span(), msg));
			}

			if type_attrs_const.len() == 1 {
				match trait_item {
					syn::TraitItem::Type(type_) => {
						let const_ = syn::parse2::<ConstMetadataDef>(type_.to_token_stream())
							.map_err(|e| {
								let error_msg = "Invalid usage of `#[pallet::const_]`, syntax \
									must be `type $SomeIdent: Get<$SomeType>;`";
								let mut err = syn::Error::new(type_.span(), error_msg);
								err.combine(e);
								err
							})?;

						consts_metadata.push(const_);
					},
					_ => {
						let msg = "Invalid pallet::const in pallet::trait, expect type trait \
							item";
						return Err(syn::Error::new(trait_item.span(), msg));
					},
				}
			}
		}

		// TODO TODO: check for frame_system bound ?

		Ok(Self { index, has_instance, consts_metadata })
	}
}
