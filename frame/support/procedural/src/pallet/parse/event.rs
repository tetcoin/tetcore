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
	syn::custom_keyword!(metadata);
	syn::custom_keyword!(Event);
}

pub struct EventDef {
	/// The index of event item in pallet module.
	pub index: usize,
	/// The keyword Event used (contains span).
	pub event: keyword::Event,
	/// Event metadatas: `(name, args, docs)`.
	pub metadata: Vec<(syn::Ident, Vec<syn::Ident>, Vec<syn::Lit>)>,
	/// A set of usage of instance, must be check for consistency with trait.
	pub instances: Vec<helper::InstanceUsage>,
	/// If event is declared with instance.
	pub has_instance: bool,
	pub is_generic: bool,
}

impl EventDef {
	/// Return the generic to be used when using Event type
	///
	/// Depending on its definition it can be: ``, `T` or `T, I`
	pub fn event_use_gen(&self) -> proc_macro2::TokenStream {
		if self.is_generic {
			if self.has_instance {
				quote::quote!(T, I)
			} else {
				quote::quote!(T)
			}
		} else {
			quote::quote!()
		}
	}

	/// Return the generic to be used in `impl<..>` when implementing on Event type.
	pub fn event_impl_gen(&self) -> proc_macro2::TokenStream {
		if self.is_generic {
			if self.has_instance {
				quote::quote!(T: Trait<I>, I: Instance)
			} else {
				quote::quote!(T: Trait)
			}
		} else {
			quote::quote!()
		}
	}
}

/// Attribute for Event: defines metadata name to use.
///
/// Syntax is:
/// `#[pallet::metadata(SomeType = MetadataName, ...)]`
pub struct PalletEventAttr {
	metadata: Vec<(syn::Type, syn::Ident)>,
	span: proc_macro2::Span,
}

/// Parse for syntax `$Type = $Ident`.
fn parse_event_metadata_element(input: syn::parse::ParseStream) -> syn::Result<(syn::Type, syn::Ident)> {
	let typ = input.parse::<syn::Type>()?;
	input.parse::<syn::Token![=]>()?;
	let ident = input.parse::<syn::Ident>()?;
	Ok((typ, ident))
}

impl syn::parse::Parse for PalletEventAttr {
	fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
		input.parse::<syn::Token![#]>()?;
		let content;
		syn::bracketed!(content in input);
		content.parse::<syn::Ident>()?;
		content.parse::<syn::Token![::]>()?;

		let lookahead = content.lookahead1();
		if lookahead.peek(keyword::metadata) {
			let span = content.parse::<keyword::metadata>()?.span();
			let metadata_content;
			syn::parenthesized!(metadata_content in content);

			let metadata = metadata_content
				.parse_terminated::<_, syn::Token![,]>(parse_event_metadata_element)?
				.into_pairs()
				.map(syn::punctuated::Pair::into_value)
				.collect();

			Ok(PalletEventAttr { metadata, span })
		} else {
			Err(lookahead.error())
		}
	}
}

impl EventDef {
	pub fn try_from(index: usize, item: &mut syn::Item) -> syn::Result<Self> {
		let item = if let syn::Item::Enum(item) = item {
			item
		} else {
			return Err(syn::Error::new(item.span(), "Invalid pallet::event, expect item enum"))
		};

		let mut event_attrs: Vec<PalletEventAttr> = helper::take_item_attrs(&mut item.attrs)?;
		if event_attrs.len() > 1 {
			let msg = "Invalid pallet::metadata, expected only one attribute \
				`pallet::metadata`";
			return Err(syn::Error::new(event_attrs[1].span, msg));
		}
		let metadata = event_attrs.pop().map_or(vec![], |attr| attr.metadata);

		if !matches!(item.vis, syn::Visibility::Public(_)) {
			let msg = "Invalid pallet::event, `Error` must be public";
			return Err(syn::Error::new(item.span(), msg));
		}
		if item.generics.where_clause.is_some() {
			let msg = "Invalid pallet::event, unexpected where clause";
			return Err(syn::Error::new(item.generics.where_clause.as_ref().unwrap().span(), msg));
		}

		let has_instance = item.generics.params.len() == 2;
		let is_generic = item.generics.params.len() > 0;

		let mut instances = vec![];
		if let Some(u) = helper::check_type_def_optional_generics(
			&item.generics,
			item.ident.span()
		)? {
			instances.push(u);
		}

		let event = syn::parse2::<keyword::Event>(item.ident.to_token_stream())?;

		let metadata = item.variants.iter()
			.map(|variant| {
				let name = variant.ident.clone();
				let docs = helper::get_doc_literals(&variant.attrs);
				let args = variant.fields.iter()
					.map(|field| {
						metadata.iter().find(|m| m.0 == field.ty)
							.map(|m| m.1.clone())
							.or_else(|| {
								if let syn::Type::Path(p) = &field.ty {
									p.path.segments.last().map(|s| s.ident.clone())
								} else {
									None
								}
							})
							.ok_or_else(|| {
								let msg = "Invalid pallet::event, type can't be parsed for \
									metadata, must be either a path type (and thus last \
									segments ident is metadata) or match a type in the \
									metadata attributes";
								syn::Error::new(field.span(), msg)
							})
					})
					.collect::<syn::Result<_>>()?;

				Ok((name, args, docs))
			})
			.collect::<syn::Result<_>>()?;

		Ok(EventDef {
			index,
			metadata,
			instances,
			has_instance,
			event,
			is_generic,
		})
	}
}
