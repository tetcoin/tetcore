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

//! Parse for pallet macro.
//!
//! Parse the module into `Def` struct through `Def::try_from` function.

pub mod trait_;
pub mod module;
pub mod module_interface;
pub mod call;
pub mod error;
pub mod origin;
pub mod inherent;
pub mod storage;
pub mod event;
pub mod helper;
pub mod genesis_config;
pub mod genesis_build;
pub mod validate_unsigned;
pub mod type_value;

use syn::spanned::Spanned;
use frame_support_procedural_tools::generate_crate_access_2018;

/// Parsed definition of a pallet.
pub struct Def {
	/// The module items.
	/// (their order must not be modified because they are registered in individual definitions).
	pub item: syn::ItemMod,
	pub trait_: trait_::TraitDef,
	pub module: module::ModuleDef,
	pub module_interface: module_interface::ModuleInterfaceDef,
	pub call: call::CallDef,
	pub storages: Vec<storage::StorageDef>,
	pub error: Option<error::ErrorDef>,
	pub event: Option<event::EventDef>,
	pub origin: Option<origin::OriginDef>,
	pub inherent: Option<inherent::InherentDef>,
	pub genesis_config: Option<genesis_config::GenesisConfigDef>,
	pub genesis_build: Option<genesis_build::GenesisBuildDef>,
	pub validate_unsigned: Option<validate_unsigned::ValidateUnsignedDef>,
	pub type_values: Vec<type_value::TypeValueDef>,
	pub frame_system: syn::Ident,
	pub frame_support: syn::Ident,
}

impl Def {
	pub fn try_from(mut item: syn::ItemMod) -> syn::Result<Self> {
		let frame_system = generate_crate_access_2018("frame-system")?;
		let frame_support = generate_crate_access_2018("frame-support")?;

		let item_span = item.span().clone();
		let items = &mut item.content.as_mut()
			.ok_or_else(|| {
				let msg = "Invalid pallet definition, expect mod to be inlined.";
				syn::Error::new(item_span, msg)
			})?.1;

		let mut trait_ = None;
		let mut module = None;
		let mut module_interface = None;
		let mut call = None;
		let mut error = None;
		let mut event = None;
		let mut origin = None;
		let mut inherent = None;
		let mut genesis_config = None;
		let mut genesis_build = None;
		let mut validate_unsigned = None;
		let mut storages = vec![];
		let mut type_values = vec![];

		for (index, item) in items.iter_mut().enumerate() {
			let pallet_attr: Option<PalletAttr> = helper::take_first_item_attr(item)?;

			match pallet_attr {
				Some(PalletAttr::Trait(_)) if trait_.is_none() =>
					trait_ = Some(trait_::TraitDef::try_from(&frame_system, index, item)?),
				Some(PalletAttr::Module(_)) if module.is_none() =>
					module = Some(module::ModuleDef::try_from(index, item)?),
				Some(PalletAttr::ModuleInterface(_)) if module_interface.is_none() => {
					let m = module_interface::ModuleInterfaceDef::try_from(index, item)?;
					module_interface = Some(m);
				},
				Some(PalletAttr::Call(span)) if call.is_none() =>
					call = Some(call::CallDef::try_from(span, index, item)?),
				Some(PalletAttr::Error(_)) if error.is_none() =>
					error = Some(error::ErrorDef::try_from(index, item)?),
				Some(PalletAttr::Event(_)) if event.is_none() =>
					event = Some(event::EventDef::try_from(index, item)?),
				Some(PalletAttr::GenesisConfig(_)) if genesis_config.is_none() => {
					genesis_config
						= Some(genesis_config::GenesisConfigDef::try_from(index, item)?);
				},
				Some(PalletAttr::GenesisBuild(_)) if genesis_build.is_none() =>
					genesis_build = Some(genesis_build::GenesisBuildDef::try_from(index, item)?),
				Some(PalletAttr::Origin(_)) if origin.is_none() =>
					origin = Some(origin::OriginDef::try_from(index, item)?),
				Some(PalletAttr::Inherent(_)) if inherent.is_none() =>
					inherent = Some(inherent::InherentDef::try_from(index, item)?),
				Some(PalletAttr::Storage(_)) =>
					storages.push(storage::StorageDef::try_from(index, item)?),
				Some(PalletAttr::ValidateUnsigned(_)) if validate_unsigned.is_none() => {
					let v = validate_unsigned::ValidateUnsignedDef::try_from(index, item)?;
					validate_unsigned = Some(v);
				},
				Some(PalletAttr::TypeValue(_)) =>
					type_values.push(type_value::TypeValueDef::try_from(index, item)?),
				Some(attr) => {
					let msg = "Invalid duplicated attribute";
					return Err(syn::Error::new(attr.span(), msg));
				},
				None => (),
			}
		}

		if genesis_config.is_some() != genesis_build.is_some() {
			let msg = format!(
				"`#[pallet::genesis_config]` and `#[pallet::genesis_build]` attributes must be \
				either both used or both not used, instead genesis_config is {} and genesis_build \
				is {}",
				genesis_config.as_ref().map_or("unused", |_| "used"),
				genesis_build.as_ref().map_or("unused", |_| "used"),
			);
			return Err(syn::Error::new(item_span, msg));
		}

		let def = Def {
			item: item,
			trait_: trait_.ok_or_else(|| syn::Error::new(item_span, "Missing `#[pallet::trait_]`"))?,
			module: module
				.ok_or_else(|| syn::Error::new(item_span, "Missing `#[pallet::module]`"))?,
			module_interface: module_interface
				.ok_or_else(|| syn::Error::new(item_span, "Missing `#[pallet::module_interface]`"))?,
			call: call.ok_or_else(|| syn::Error::new(item_span, "Missing `#[pallet::call]"))?,
			genesis_config,
			genesis_build,
			validate_unsigned,
			error,
			event,
			origin,
			inherent,
			storages,
			type_values,
			frame_system,
			frame_support,
		};

		def.check_instance_usage()?;
		def.check_event_usage()?;

		Ok(def)
	}

	/// Check that usage of trait `Event` is consistent with the definition, i.e. it is declared
	/// and trait defines type Event, or not declared and no trait associated type.
	fn check_event_usage(&self) -> syn::Result<()> {
		match (
			self.trait_.has_event_type,
			self.event.is_some(),
		) {
			(true, false) => {
				let msg = "Invalid usage of Event, trait `Trait` contains associated type `Event`, \
					but enum `Event` is not declared (i.e. no use of `#[pallet::event]`). \
					Note that type `Event` in trait is reserved to work alongside pallet event.";
				Err(syn::Error::new(proc_macro2::Span::call_site(), msg))
			},
			(false, true) => {
				let msg = "Invalid usage of Event, trait `Trait` contains no associated type \
					`Event`, but enum `Event` is declared (in use of `#[pallet::event]`). \
					An Event associated type must be declare on trait `Trait`.";
				Err(syn::Error::new(proc_macro2::Span::call_site(), msg))
			},
			_ => Ok(())
		}
	}
	/// Check that usage of trait `Trait` is consistent with the definition, i.e. it is used with
	/// instance iff it is defined with instance.
	fn check_instance_usage(&self) -> syn::Result<()> {
		let mut instances = vec![];
		instances.extend_from_slice(&self.call.instances[..]);
		instances.extend_from_slice(&self.module.instances[..]);
		instances.extend_from_slice(&self.module_interface.instances[..]);
		instances.extend(&mut self.storages.iter().flat_map(|s| s.instances.clone()));
		if let Some(event) = &self.event {
			instances.extend_from_slice(&event.instances[..]);
		}
		if let Some(error) = &self.error {
			instances.extend_from_slice(&error.instances[..]);
		}
		if let Some(inherent) = &self.inherent {
			instances.extend_from_slice(&inherent.instances[..]);
		}
		if let Some(origin) = &self.origin {
			instances.extend_from_slice(&origin.instances[..]);
		}
		if let Some(genesis_config) = &self.genesis_config {
			instances.extend_from_slice(&genesis_config.instances[..]);
		}
		if let Some(genesis_build) = &self.genesis_build {
			instances.extend_from_slice(&genesis_build.instances[..]);
		}

		let mut errors = instances.into_iter()
			.filter_map(|instances| {
				if instances.has_instance == self.trait_.has_instance {
					return None
				}
				let msg = if self.trait_.has_instance {
					"Invalid generic declaration, trait is defined with instance but generic use none"
				} else {
					"Invalid generic declaration, trait is defined without instance but generic use \
						some"
				};
				Some(syn::Error::new(instances.span, msg))
			});

		if let Some(mut first_error) = errors.next() {
			for error in errors {
				first_error.combine(error)
			}
			Err(first_error)
		} else {
			Ok(())
		}
	}

	/// Depending on if pallet is instantiable:
	/// * either `T: Trait`
	/// * or `T: Trait<I>, I: 'static`
	pub fn type_impl_generics(&self) -> proc_macro2::TokenStream {
		if self.trait_.has_instance {
			quote::quote!(T: Trait<I>, I: 'static)
		} else {
			quote::quote!(T: Trait)
		}
	}

	/// Depending on if pallet is instantiable:
	/// * either `T: Trait`
	/// * or `T: Trait<I>, I: 'static = ()`
	pub fn type_decl_generics(&self) -> proc_macro2::TokenStream {
		if self.trait_.has_instance {
			quote::quote!(T: Trait<I>, I: 'static = ())
		} else {
			quote::quote!(T: Trait)
		}
	}

	/// Depending on if pallet is instantiable:
	/// * either ``
	/// * or `<I>`
	/// to be used when using pallet trait `Trait`
	pub fn trait_use_generics(&self) -> proc_macro2::TokenStream {
		if self.trait_.has_instance {
			quote::quote!(<I>)
		} else {
			quote::quote!()
		}
	}

	/// Depending on if pallet is instantiable:
	/// * either `T`
	/// * or `T, I`
	pub fn type_use_generics(&self) -> proc_macro2::TokenStream {
		if self.trait_.has_instance {
			quote::quote!(T, I)
		} else {
			quote::quote!(T)
		}
	}

	/// Return path to frame-support crate.
	pub fn scrate(&self) -> syn::Ident {
		self.frame_support.clone()
	}

	/// Return path to frame-system crate.
	pub fn system_crate(&self) -> syn::Ident {
		self.frame_system.clone()
	}
}

/// List of additional token to be used for parsing.
mod keyword {
	syn::custom_keyword!(origin);
	syn::custom_keyword!(call);
	syn::custom_keyword!(event);
	syn::custom_keyword!(module);
	syn::custom_keyword!(trait_);
	syn::custom_keyword!(module_interface);
	syn::custom_keyword!(inherent);
	syn::custom_keyword!(error);
	syn::custom_keyword!(storage);
	syn::custom_keyword!(genesis_build);
	syn::custom_keyword!(genesis_config);
	syn::custom_keyword!(validate_unsigned);
	syn::custom_keyword!(type_value);
	syn::custom_keyword!(pallet);
	syn::custom_keyword!(generate_store);
	syn::custom_keyword!(Store);
}

/// Parse attributes for item in pallet module
/// syntax must be `pallet::` (e.g. `#[pallet::trait_]`)
enum PalletAttr {
	Trait(proc_macro2::Span),
	Module(proc_macro2::Span),
	ModuleInterface(proc_macro2::Span),
	Call(proc_macro2::Span),
	Error(proc_macro2::Span),
	Event(proc_macro2::Span),
	Origin(proc_macro2::Span),
	Inherent(proc_macro2::Span),
	Storage(proc_macro2::Span),
	GenesisConfig(proc_macro2::Span),
	GenesisBuild(proc_macro2::Span),
	ValidateUnsigned(proc_macro2::Span),
	TypeValue(proc_macro2::Span),
}

impl PalletAttr {
	fn span(&self) -> proc_macro2::Span {
		match self {
			Self::Trait(span) => span.clone(),
			Self::Module(span) => span.clone(),
			Self::ModuleInterface(span) => span.clone(),
			Self::Call(span) => span.clone(),
			Self::Error(span) => span.clone(),
			Self::Event(span) => span.clone(),
			Self::Origin(span) => span.clone(),
			Self::Inherent(span) => span.clone(),
			Self::Storage(span) => span.clone(),
			Self::GenesisConfig(span) => span.clone(),
			Self::GenesisBuild(span) => span.clone(),
			Self::ValidateUnsigned(span) => span.clone(),
			Self::TypeValue(span) => span.clone(),
		}
	}
}

impl syn::parse::Parse for PalletAttr {
	fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
		input.parse::<syn::Token![#]>()?;
		let content;
		syn::bracketed!(content in input);
		content.parse::<keyword::pallet>()?;
		content.parse::<syn::Token![::]>()?;

		let lookahead = content.lookahead1();
		if lookahead.peek(keyword::trait_) {
			Ok(PalletAttr::Trait(content.parse::<keyword::trait_>()?.span()))
		} else if lookahead.peek(keyword::module) {
			Ok(PalletAttr::Module(content.parse::<keyword::module>()?.span()))
		} else if lookahead.peek(keyword::module_interface) {
			Ok(PalletAttr::ModuleInterface(content.parse::<keyword::module_interface>()?.span()))
		} else if lookahead.peek(keyword::call) {
			Ok(PalletAttr::Call(content.parse::<keyword::call>()?.span()))
		} else if lookahead.peek(keyword::error) {
			Ok(PalletAttr::Error(content.parse::<keyword::error>()?.span()))
		} else if lookahead.peek(keyword::event) {
			Ok(PalletAttr::Event(content.parse::<keyword::event>()?.span()))
		} else if lookahead.peek(keyword::origin) {
			Ok(PalletAttr::Origin(content.parse::<keyword::origin>()?.span()))
		} else if lookahead.peek(keyword::inherent) {
			Ok(PalletAttr::Inherent(content.parse::<keyword::inherent>()?.span()))
		} else if lookahead.peek(keyword::storage) {
			Ok(PalletAttr::Storage(content.parse::<keyword::storage>()?.span()))
		} else if lookahead.peek(keyword::genesis_config) {
			Ok(PalletAttr::GenesisConfig(content.parse::<keyword::genesis_config>()?.span()))
		} else if lookahead.peek(keyword::genesis_build) {
			Ok(PalletAttr::GenesisBuild(content.parse::<keyword::genesis_build>()?.span()))
		} else if lookahead.peek(keyword::validate_unsigned) {
			Ok(PalletAttr::ValidateUnsigned(content.parse::<keyword::validate_unsigned>()?.span()))
		} else if lookahead.peek(keyword::type_value) {
			Ok(PalletAttr::TypeValue(content.parse::<keyword::type_value>()?.span()))
		} else {
			Err(lookahead.error())
		}
	}
}
