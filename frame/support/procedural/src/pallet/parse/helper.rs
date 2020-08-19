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
use quote::ToTokens;

/// List of additional token to be used for parsing.
mod keyword {
	syn::custom_keyword!(I);
	syn::custom_keyword!(compact);
	syn::custom_keyword!(GenesisBuilder);
	syn::custom_keyword!(OriginFor);
	syn::custom_keyword!(Trait);
	syn::custom_keyword!(T);
	syn::custom_keyword!(Instance);
	syn::custom_keyword!(DefaultInstance);
	syn::custom_keyword!(Module);
	syn::custom_keyword!(origin);
}

/// A usage of instance, either the trait `Trait` has been used with instance or without instance.
/// Used to check for consistency.
#[derive(Clone)]
pub struct InstanceUsage {
	pub has_instance: bool,
	pub span: proc_macro2::Span,
}

/// Trait implemented for syn items to get mutable references on their attributes.
///
/// NOTE: verbatim variants are not supported.
pub trait MutItemAttrs {
	fn mut_item_attrs(&mut self) -> Option<&mut Vec<syn::Attribute>>;
}

/// Take the first pallet attribute (e.g. attribute like `#[pallet..]`) and decode it to `Attr`
pub fn take_first_item_attr<Attr>(item: &mut impl MutItemAttrs) -> syn::Result<Option<Attr>> where
	Attr: syn::parse::Parse,
{
	let attrs = if let Some(attrs) = item.mut_item_attrs() {
		attrs
	} else {
		return Ok(None)
	};

	if let Some(index) = attrs.iter()
		.position(|attr|
			attr.path.segments.first().map_or(false, |segment| segment.ident == "pallet")
		)
	{
		let pallet_attr = attrs.remove(index);
		Ok(Some(syn::parse2(pallet_attr.into_token_stream())?))
	} else {
		Ok(None)
	}
}

/// Take all the pallet attributes (e.g. attribute like `#[pallet..]`) and decode them to `Attr`
pub fn take_item_attrs<Attr>(item: &mut impl MutItemAttrs) -> syn::Result<Vec<Attr>> where
	Attr: syn::parse::Parse,
{
	let mut pallet_attrs = Vec::new();

	while let Some(attr) = take_first_item_attr(item)? {
		pallet_attrs.push(attr)
	}

	Ok(pallet_attrs)
}

impl MutItemAttrs for syn::Item {
	fn mut_item_attrs(&mut self) -> Option<&mut Vec<syn::Attribute>> {
		match self {
			Self::Const(item) => Some(item.attrs.as_mut()),
			Self::Enum(item) => Some(item.attrs.as_mut()),
			Self::ExternCrate(item) => Some(item.attrs.as_mut()),
			Self::Fn(item) => Some(item.attrs.as_mut()),
			Self::ForeignMod(item) => Some(item.attrs.as_mut()),
			Self::Impl(item) => Some(item.attrs.as_mut()),
			Self::Macro(item) => Some(item.attrs.as_mut()),
			Self::Macro2(item) => Some(item.attrs.as_mut()),
			Self::Mod(item) => Some(item.attrs.as_mut()),
			Self::Static(item) => Some(item.attrs.as_mut()),
			Self::Struct(item) => Some(item.attrs.as_mut()),
			Self::Trait(item) => Some(item.attrs.as_mut()),
			Self::TraitAlias(item) => Some(item.attrs.as_mut()),
			Self::Type(item) => Some(item.attrs.as_mut()),
			Self::Union(item) => Some(item.attrs.as_mut()),
			Self::Use(item) => Some(item.attrs.as_mut()),
			Self::Verbatim(_) => None,
			Self::__Nonexhaustive => None,
		}
	}
}


impl MutItemAttrs for syn::TraitItem {
	fn mut_item_attrs(&mut self) -> Option<&mut Vec<syn::Attribute>> {
		match self {
			Self::Const(item) => Some(item.attrs.as_mut()),
			Self::Method(item) => Some(item.attrs.as_mut()),
			Self::Type(item) => Some(item.attrs.as_mut()),
			Self::Macro(item) => Some(item.attrs.as_mut()),
			Self::Verbatim(_) => None,
			Self::__Nonexhaustive => None,
		}
	}
}

impl MutItemAttrs for Vec<syn::Attribute> {
	fn mut_item_attrs(&mut self) -> Option<&mut Vec<syn::Attribute>> {
		Some(self)
	}
}

/// Return all doc attributes literals found.
pub fn get_doc_literals(attrs: &Vec<syn::Attribute>) -> Vec<syn::Lit> {
	attrs.iter()
		.filter_map(|attr| {
			if let Ok(syn::Meta::NameValue(meta)) = attr.parse_meta() {
				if meta.path.get_ident().map_or(false, |ident| ident == "doc") {
					Some(meta.lit.clone())
				} else {
					None
				}
			} else {
				None
			}
		})
		.collect()
}

/// Check the syntax: `I: Instance = DefaultInstance`
///
/// `span` is used in case generics is empty (empty generics has span == call_site).
///
/// return the instance if found.
pub fn check_trait_def_generics(
	gen: &syn::Generics,
	span: proc_macro2::Span,
) -> syn::Result<()> {
	let expected = "expect `I: Instance = DefaultInstance`";
	pub struct CheckTraitDefGenerics;
	impl syn::parse::Parse for CheckTraitDefGenerics {
		fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
			input.parse::<keyword::I>()?;
			input.parse::<syn::Token![:]>()?;
			input.parse::<keyword::Instance>()?;
			input.parse::<syn::Token![=]>()?;
			input.parse::<keyword::DefaultInstance>()?;

			Ok(Self)
		}
	}

	syn::parse2::<CheckTraitDefGenerics>(gen.params.to_token_stream())
		.map_err(|e| {
			let msg = format!("Invalid generics: {}", expected);
			let mut err = syn::Error::new(span, msg);
			err.combine(e);
			err
		})?;

	Ok(())
}

/// Check the syntax:
/// * either `T`
/// * or `T, I = DefaultInstance`
///
/// `span` is used in case generics is empty (empty generics has span == call_site).
///
/// return the instance if found.
pub fn check_type_def_generics(
	gen: &syn::Generics,
	span: proc_macro2::Span,
) -> syn::Result<InstanceUsage> {
	let expected = "expect `T` or `T, I = DefaultInstance`";
	pub struct Checker(InstanceUsage);
	impl syn::parse::Parse for Checker {
		fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
			let mut instance_usage = InstanceUsage {
				has_instance: false,
				span: input.span(),
			};

			input.parse::<keyword::T>()?;
			if input.peek(syn::Token![,]) {
				instance_usage.has_instance = true;
				input.parse::<syn::Token![,]>()?;
				input.parse::<keyword::I>()?;
				input.parse::<syn::Token![=]>()?;
				input.parse::<keyword::DefaultInstance>()?;
			}

			Ok(Self(instance_usage))
		}
	}

	let i = syn::parse2::<Checker>(gen.params.to_token_stream())
		.map_err(|e| {
			let msg = format!("Invalid type def generics: {}", expected);
			let mut err = syn::Error::new(span, msg);
			err.combine(e);
			err
		})?.0;

	Ok(i)
}

/// Check the syntax:
/// * either `` (no generics
/// * or `T`
/// * or `T: Trait`
/// * or `T, I = DefaultInstance`
/// * or `T: Trait<I>, I: Instance = DefaultInstance`
///
/// `span` is used in case generics is empty (empty generics has span == call_site).
///
/// return the instance if found.
pub fn check_type_def_optional_generics(
	gen: &syn::Generics,
	span: proc_macro2::Span,
) -> syn::Result<Option<InstanceUsage>> {
	let expected = "expect `` or `T` or `T: Trait` or `T, I = DefaultInstance` or \
		`T: Trait<I>, I: Instance = DefaultInstance`";
	pub struct Checker(Option<InstanceUsage>);
	impl syn::parse::Parse for Checker {
		fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
			if input.is_empty() {
				return Ok(Self(None))
			}

			let mut instance_usage = InstanceUsage {
				span: input.span(),
				has_instance: false,
			};

			input.parse::<keyword::T>()?;

			if input.is_empty() {
				return Ok(Self(Some(instance_usage)))
			}

			let lookahead = input.lookahead1();
			if lookahead.peek(syn::Token![,]) {
				instance_usage.has_instance = true;
				input.parse::<syn::Token![,]>()?;
				input.parse::<keyword::I>()?;
				input.parse::<syn::Token![=]>()?;
				input.parse::<keyword::DefaultInstance>()?;

				Ok(Self(Some(instance_usage)))
			} else if lookahead.peek(syn::Token![:]) {
				input.parse::<syn::Token![:]>()?;
				input.parse::<keyword::Trait>()?;

				if input.is_empty() {
					return Ok(Self(Some(instance_usage)))
				}

				instance_usage.has_instance = true;
				input.parse::<syn::Token![<]>()?;
				input.parse::<keyword::I>()?;
				input.parse::<syn::Token![>]>()?;
				input.parse::<syn::Token![,]>()?;
				input.parse::<keyword::I>()?;
				input.parse::<syn::Token![:]>()?;
				input.parse::<keyword::Instance>()?;
				input.parse::<syn::Token![=]>()?;
				input.parse::<keyword::DefaultInstance>()?;

				Ok(Self(Some(instance_usage)))
			} else {
				Err(lookahead.error())
			}
		}
	}

	let i = syn::parse2::<Checker>(gen.params.to_token_stream())
		.map_err(|e| {
			let msg = format!("Invalid type def generics: {}", expected);
			let mut err = syn::Error::new(span, msg);
			err.combine(e);
			err
		})?.0;

	Ok(i)
}

/// Check the syntax: `origin: OriginFor<T>`
pub fn check_dispatchable_first_arg(arg: &syn::FnArg) -> syn::Result<()> {
	let expected = "expect `origin: OriginFor<T>`";

	pub struct CheckDispatchableFirstArg;
	impl syn::parse::Parse for CheckDispatchableFirstArg {
		fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
			input.parse::<keyword::origin>()?;
			input.parse::<syn::Token![:]>()?;
			input.parse::<keyword::OriginFor>()?;
			input.parse::<syn::Token![<]>()?;
			input.parse::<keyword::T>()?;
			input.parse::<syn::Token![>]>()?;

			Ok(Self)
		}
	}

	syn::parse2::<CheckDispatchableFirstArg>(arg.to_token_stream())
		.map_err(|e| {
			let msg = format!("Invalid arg: {}", expected);
			let mut err = syn::Error::new(arg.span(), msg);
			err.combine(e);
			err
		})?;

	Ok(())
}

/// Check the syntax:
/// * either `Module<T>`
/// * or `Module<T, I>`
///
/// return the instance if found.
pub fn check_module_usage(type_: &Box<syn::Type>) -> syn::Result<InstanceUsage> {
	let expected = "expect `Module<T>` or `Module<T, I>`";
	pub struct Checker(InstanceUsage);
	impl syn::parse::Parse for Checker {
		fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
			let mut instance_usage = InstanceUsage {
				span: input.span(),
				has_instance: false,
			};

			input.parse::<keyword::Module>()?;
			input.parse::<syn::Token![<]>()?;
			input.parse::<keyword::T>()?;
			if input.peek(syn::Token![,]) {
				instance_usage.has_instance = true;
				input.parse::<syn::Token![,]>()?;
				input.parse::<keyword::I>()?;
			}
			input.parse::<syn::Token![>]>()?;

			Ok(Self(instance_usage))
		}
	}

	let i = syn::parse2::<Checker>(type_.to_token_stream())
		.map_err(|e| {
			let msg = format!("Invalid module type: {}", expected);
			let mut err = syn::Error::new(type_.span(), msg);
			err.combine(e);
			err
		})?.0;

	Ok(i)
}

/// Check the generic is:
/// * either `T: Trait`
/// * or `T: Trait<I>, I: Instance`
///
/// `span` is used in case generics is empty (empty generics has span == call_site).
///
/// return weither it contains instance.
pub fn check_impl_generics(
	gen: &syn::Generics,
	span: proc_macro2::Span
) -> syn::Result<InstanceUsage> {
	let expected = "expect `impl<T: Trait>` or `impl<T: Trait<I>, I: Instance>`";
	pub struct Checker(InstanceUsage);
	impl syn::parse::Parse for Checker {
		fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
			let mut instance_usage = InstanceUsage {
				span: input.span(),
				has_instance: false,
			};

			input.parse::<keyword::T>()?;
			input.parse::<syn::Token![:]>()?;
			input.parse::<keyword::Trait>()?;
			if input.peek(syn::Token![<]) {
				instance_usage.has_instance = true;
				input.parse::<syn::Token![<]>()?;
				input.parse::<keyword::I>()?;
				input.parse::<syn::Token![>]>()?;
				input.parse::<syn::Token![,]>()?;
				input.parse::<keyword::I>()?;
				input.parse::<syn::Token![:]>()?;
				input.parse::<keyword::Instance>()?;
			}

			Ok(Self(instance_usage))
		}
	}

	let i = syn::parse2::<Checker>(gen.params.to_token_stream())
		.map_err(|e| {
			let mut err = syn::Error::new(span, format!("Invalid generics: {}", expected));
			err.combine(e);
			err
		})?.0;

	Ok(i)
}

/// Check the syntax:
/// * either `` (no generics
/// * or `T`
/// * or `T: Trait`
/// * or `T, I = DefaultInstance`
/// * or `T: Trait<I>, I = DefaultInstance`
/// * or `T: Trait<I>, I: Instance = DefaultInstance`
/// * or `I = DefaultInstance`
/// * or `I: Instance = DefaultInstance`
///
/// `span` is used in case generics is empty (empty generics has span == call_site).
///
/// return the instance if found.
pub fn check_storage_optional_gen(
	gen: &syn::Generics,
	span: proc_macro2::Span,
) -> syn::Result<InstanceUsage> {
	let expected = "expect `` or `T` or `T: Trait` or `T, I = DefaultInstance` or \
		`T: Trait<I>, I = DefaultInstance` or `T: Trait<I>, I: Instance = DefaultInstance` or \
		`I = DefaultInstance` or `I: Instance = DefaultInstance`";
	pub struct Checker(InstanceUsage);
	impl syn::parse::Parse for Checker {
		fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
			let mut instance_usage = InstanceUsage {
				span: input.span(),
				has_instance: false,
			};

			if input.is_empty() {
				return Ok(Self(instance_usage))
			}

			let lookahead = input.lookahead1();
			if lookahead.peek(keyword::I) {
				instance_usage.has_instance = true;

				input.parse::<keyword::I>()?;

				if input.peek(syn::Token![:]) {
					input.parse::<syn::Token![:]>()?;
					input.parse::<keyword::Instance>()?;
				}

				input.parse::<syn::Token![=]>()?;
				input.parse::<keyword::DefaultInstance>()?;

				Ok(Self(instance_usage))
			} else if lookahead.peek(keyword::T) {
				input.parse::<keyword::T>()?;

				if input.is_empty() {
					return Ok(Self(instance_usage))
				}

				let lookahead = input.lookahead1();
				if lookahead.peek(syn::Token![,]) {
					instance_usage.has_instance = true;
					input.parse::<syn::Token![,]>()?;
					input.parse::<keyword::I>()?;
					input.parse::<syn::Token![=]>()?;
					input.parse::<keyword::DefaultInstance>()?;

					Ok(Self(instance_usage))
				} else if lookahead.peek(syn::Token![:]) {
					input.parse::<syn::Token![:]>()?;
					input.parse::<keyword::Trait>()?;

					if input.is_empty() {
						return Ok(Self(instance_usage))
					}

					instance_usage.has_instance = true;
					input.parse::<syn::Token![<]>()?;
					input.parse::<keyword::I>()?;
					input.parse::<syn::Token![>]>()?;
					input.parse::<syn::Token![,]>()?;
					input.parse::<keyword::I>()?;


					if input.peek(syn::Token![:]) {
						input.parse::<syn::Token![:]>()?;
						input.parse::<keyword::Instance>()?;
					}

					input.parse::<syn::Token![=]>()?;
					input.parse::<keyword::DefaultInstance>()?;

					Ok(Self(instance_usage))
				} else {
					Err(lookahead.error())
				}
			} else {
				Err(lookahead.error())
			}
		}
	}

	let i = syn::parse2::<Checker>(gen.params.to_token_stream())
		.map_err(|e| {
			let msg = format!("Invalid type def generics: {}", expected);
			let mut err = syn::Error::new(span, msg);
			err.combine(e);
			err
		})?.0;

	Ok(i)
}

/// Check the syntax:
/// * either `GenesisBuilder<T>`
/// * or `GenesisBuilder<T, I>`
///
/// return the instance if found.
pub fn check_genesis_builder_usage(type_: &syn::Path) -> syn::Result<InstanceUsage> {
	let expected = "expect `GenesisBuilder<T>` or `GenesisBuilder<T, I>`";
	pub struct Checker(InstanceUsage);
	impl syn::parse::Parse for Checker {
		fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
			let mut instance_usage = InstanceUsage {
				span: input.span(),
				has_instance: false,
			};

			input.parse::<keyword::GenesisBuilder>()?;
			input.parse::<syn::Token![<]>()?;
			input.parse::<keyword::T>()?;
			if input.peek(syn::Token![,]) {
				instance_usage.has_instance = true;
				input.parse::<syn::Token![,]>()?;
				input.parse::<keyword::I>()?;
			}
			input.parse::<syn::Token![>]>()?;

			Ok(Self(instance_usage))
		}
	}

	let i = syn::parse2::<Checker>(type_.to_token_stream())
		.map_err(|e| {
			let msg = format!("Invalid genesis builder: {}", expected);
			let mut err = syn::Error::new(type_.span(), msg);
			err.combine(e);
			err
		})?.0;

	Ok(i)
}
