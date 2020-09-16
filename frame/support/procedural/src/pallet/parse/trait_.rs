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
	syn::custom_keyword!(From);
	syn::custom_keyword!(T);
	syn::custom_keyword!(I);
	syn::custom_keyword!(Get);
	syn::custom_keyword!(trait_);
	syn::custom_keyword!(IsType);
	syn::custom_keyword!(Event);
	syn::custom_keyword!(const_);
	syn::custom_keyword!(frame_system);
	syn::custom_keyword!(disable_frame_system_supertrait_check);
}

/// Input definition for the pallet trait.
pub struct TraitDef {
	/// The index of error item in pallet module.
	pub index: usize,
	/// Wheither the trait has instance (i.e. define with `Trait<I: Instance = DefaultInstance>`)
	pub has_instance: bool,
	/// Const associated type.
	pub consts_metadata: Vec<ConstMetadataDef>,
	/// Wether the trait has the associated type `Event`, note that those bounds are checked:
	/// * `IsType<Self as frame_system::Trait>::Event`
	/// * `From<Event>` or `From<Event<T>>` or `From<Event<T, I>>`
	pub has_event_type: bool,
}

/// Input definition for a constant in pallet trait.
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

/// Parse for `#[pallet::disable_frame_system_supertrait_check]`
pub struct DisableFrameSystemSupertraitCheck;

impl syn::parse::Parse for DisableFrameSystemSupertraitCheck {
	fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
		input.parse::<syn::Token![#]>()?;
		let content;
		syn::bracketed!(content in input);
		content.parse::<syn::Ident>()?;
		content.parse::<syn::Token![::]>()?;

		content.parse::<keyword::disable_frame_system_supertrait_check>()?;
		Ok(Self)
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

/// Parse for `$ident::Trait`
pub struct TraitBoundParse(syn::Ident);

impl syn::parse::Parse for TraitBoundParse {
	fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
		let ident = input.parse::<syn::Ident>()?;
		input.parse::<syn::Token![::]>()?;
		input.parse::<keyword::Trait>()?;

		Ok(Self(ident))
	}
}

/// Parse for `IsType<<Sef as $ident::Trait>::Event>`
pub struct IsTypeBoundEventParse(syn::Ident);

impl syn::parse::Parse for IsTypeBoundEventParse {
	fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
		input.parse::<keyword::IsType>()?;
		input.parse::<syn::Token![<]>()?;
		input.parse::<syn::Token![<]>()?;
		input.parse::<syn::Token![Self]>()?;
		input.parse::<syn::Token![as]>()?;
		let ident = input.parse::<syn::Ident>()?;
		input.parse::<syn::Token![::]>()?;
		input.parse::<keyword::Trait>()?;
		input.parse::<syn::Token![>]>()?;
		input.parse::<syn::Token![::]>()?;
		input.parse::<keyword::Event>()?;
		input.parse::<syn::Token![>]>()?;

		Ok(Self(ident))
	}
}

/// Parse for `From<Event>` or `From<Event<Self>>` or `From<Event<Self, I>>`
pub struct FromEventParse {
	is_generic: bool,
	has_instance: bool,
}

impl syn::parse::Parse for FromEventParse {
	fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
		let mut is_generic = false;
		let mut has_instance = false;

		input.parse::<keyword::From>()?;
		input.parse::<syn::Token![<]>()?;
		input.parse::<keyword::Event>()?;
		if input.peek(syn::Token![<]) {
			is_generic = true;
			input.parse::<syn::Token![<]>()?;
			input.parse::<syn::Token![Self]>()?;
			if input.peek(syn::Token![,]) {
				input.parse::<syn::Token![,]>()?;
				input.parse::<keyword::I>()?;
				has_instance = true;
			}
			input.parse::<syn::Token![>]>()?;
		}
		input.parse::<syn::Token![>]>()?;

		Ok(Self { is_generic, has_instance })
	}
}

/// Check if trait_item is `type Event`, if so checks its bounds are those expected.
/// (Event type is reserved type)
fn check_event_type(
	frame_system: &syn::Ident,
	trait_item: &syn::TraitItem,
	trait_has_instance: bool
)  -> syn::Result<bool> {
	if let syn::TraitItem::Type(type_) = trait_item {
		if type_.ident == "Event" {
			// Check event has no generics
			if !type_.generics.params.is_empty() || type_.generics.where_clause.is_some() {
				let msg = "Invalid `type Event`, associated type `Event` is reserved and must have\
					no generics nor where_clause";
				return Err(syn::Error::new(trait_item.span(), msg));
			}
			// Check bound contains IsType and From

			let has_is_type_bound = type_.bounds.iter().any(|s| {
				syn::parse2::<IsTypeBoundEventParse>(s.to_token_stream())
					.map_or(false, |b| b.0 == *frame_system)
			});

			if !has_is_type_bound {
				let msg = format!(
					"Invalid `type Event`, associated type `Event` is reserved and must \
					bound: `IsType<<Self as {}::Trait>::Event>`",
					frame_system,
				);
				return Err(syn::Error::new(type_.span(), msg));
			}

			let from_event_bound = type_.bounds.iter().find_map(|s| {
				syn::parse2::<FromEventParse>(s.to_token_stream()).ok()
			});

			let from_event_bound = if let Some(b) = from_event_bound {
				b
			} else {
				let msg = "Invalid `type Event`, associated type `Event` is reserved and must \
					bound: `From<Event>` or `From<Event<Self>>` or `From<Event<Self, I>>`";
				return Err(syn::Error::new(type_.span(), msg));
			};

			if from_event_bound.is_generic
				&& (from_event_bound.has_instance != trait_has_instance)
			{
				let msg = "Invalid `type Event`, associated type `Event` bounds inconsistent \
					`From<Event..>`. Trait and generic Event must be both with instance or \
					without instance";
				return Err(syn::Error::new(type_.span(), msg));
			}

			Ok(true)
		} else {
			Ok(false)
		}
	} else {
		Ok(false)
	}
}

impl TraitDef {
	pub fn try_from(
		frame_system: &syn::Ident,
		index: usize,
		item: &mut syn::Item
	) -> syn::Result<Self> {
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

		let mut has_event_type = false;
		let mut consts_metadata = vec![];
		for trait_item in &mut item.items {
			// Parse for event
			has_event_type = has_event_type
				|| check_event_type(frame_system, trait_item, has_instance)?;

			// Parse for const_
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

		let attr: Option<DisableFrameSystemSupertraitCheck> = helper::take_first_item_attr(
			&mut item.attrs
		)?;

		let disable_system_supertrait_check = attr.is_some();

		let has_frame_system_supertrait = item.supertraits.iter().any(|s| {
			syn::parse2::<TraitBoundParse>(s.to_token_stream())
				.map_or(false, |b| b.0 == *frame_system)
		});

		if !has_frame_system_supertrait && !disable_system_supertrait_check {
			let found = if item.supertraits.is_empty() {
				"none".to_string()
			} else {
				let mut found = item.supertraits.iter()
					.fold(String::new(), |acc, s| {
						format!("{}`{}`, ", acc, quote::quote!(#s).to_string())
					});
				found.pop();
				found.pop();
				found
			};

			let msg = format!(
				"Invalid pallet::trait, expect explicit `{}::Trait` as supertrait, \
				found {}. To disable this check, use \
				`#[pallet::disable_frame_system_supertrait_check]`",
				frame_system,
				found,
			);
			return Err(syn::Error::new(item.span(), msg));
		}

		Ok(Self { index, has_instance, consts_metadata, has_event_type })
	}
}
