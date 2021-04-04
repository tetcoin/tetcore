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

//! Parse for noble macro.
//!
//! Parse the module into `Def` struct through `Def::try_from` function.

pub mod config;
pub mod noble_struct;
pub mod hooks;
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
pub mod extra_constants;

use syn::spanned::Spanned;
use fabric_support_procedural_tools::generate_crate_access_2018;

/// Parsed definition of a noble.
pub struct Def {
	/// The module items.
	/// (their order must not be modified because they are registered in individual definitions).
	pub item: syn::ItemMod,
	pub config: config::ConfigDef,
	pub noble_struct: noble_struct::NobleStructDef,
	pub hooks: hooks::HooksDef,
	pub call: call::CallDef,
	pub storages: Vec<storage::StorageDef>,
	pub error: Option<error::ErrorDef>,
	pub event: Option<event::EventDef>,
	pub origin: Option<origin::OriginDef>,
	pub inherent: Option<inherent::InherentDef>,
	pub genesis_config: Option<genesis_config::GenesisConfigDef>,
	pub genesis_build: Option<genesis_build::GenesisBuildDef>,
	pub validate_unsigned: Option<validate_unsigned::ValidateUnsignedDef>,
	pub extra_constants: Option<extra_constants::ExtraConstantsDef>,
	pub type_values: Vec<type_value::TypeValueDef>,
	pub fabric_system: syn::Ident,
	pub fabric_support: syn::Ident,
}

impl Def {
	pub fn try_from(mut item: syn::ItemMod) -> syn::Result<Self> {
		let fabric_system = generate_crate_access_2018("fabric-system")?;
		let fabric_support = generate_crate_access_2018("fabric-support")?;

		let item_span = item.span();
		let items = &mut item.content.as_mut()
			.ok_or_else(|| {
				let msg = "Invalid noble definition, expected mod to be inlined.";
				syn::Error::new(item_span, msg)
			})?.1;

		let mut config = None;
		let mut noble_struct = None;
		let mut hooks = None;
		let mut call = None;
		let mut error = None;
		let mut event = None;
		let mut origin = None;
		let mut inherent = None;
		let mut genesis_config = None;
		let mut genesis_build = None;
		let mut validate_unsigned = None;
		let mut extra_constants = None;
		let mut storages = vec![];
		let mut type_values = vec![];

		for (index, item) in items.iter_mut().enumerate() {
			let noble_attr: Option<NobleAttr> = helper::take_first_item_attr(item)?;

			match noble_attr {
				Some(NobleAttr::Config(span)) if config.is_none() =>
					config = Some(config::ConfigDef::try_from(&fabric_system, span, index, item)?),
				Some(NobleAttr::Noble(span)) if noble_struct.is_none() => {
					let p = noble_struct::NobleStructDef::try_from(span, index, item)?;
					noble_struct = Some(p);
				},
				Some(NobleAttr::Hooks(span)) if hooks.is_none() => {
					let m = hooks::HooksDef::try_from(span, index, item)?;
					hooks = Some(m);
				},
				Some(NobleAttr::Call(span)) if call.is_none() =>
					call = Some(call::CallDef::try_from(span, index, item)?),
				Some(NobleAttr::Error(span)) if error.is_none() =>
					error = Some(error::ErrorDef::try_from(span, index, item)?),
				Some(NobleAttr::Event(span)) if event.is_none() =>
					event = Some(event::EventDef::try_from(span, index, item)?),
				Some(NobleAttr::GenesisConfig(_)) if genesis_config.is_none() => {
					let g = genesis_config::GenesisConfigDef::try_from(index, item)?;
					genesis_config = Some(g);
				},
				Some(NobleAttr::GenesisBuild(span)) if genesis_build.is_none() => {
					let g = genesis_build::GenesisBuildDef::try_from(span, index, item)?;
					genesis_build = Some(g);
				},
				Some(NobleAttr::Origin(_)) if origin.is_none() =>
					origin = Some(origin::OriginDef::try_from(index, item)?),
				Some(NobleAttr::Inherent(_)) if inherent.is_none() =>
					inherent = Some(inherent::InherentDef::try_from(index, item)?),
				Some(NobleAttr::Storage(span)) =>
					storages.push(storage::StorageDef::try_from(span, index, item)?),
				Some(NobleAttr::ValidateUnsigned(_)) if validate_unsigned.is_none() => {
					let v = validate_unsigned::ValidateUnsignedDef::try_from(index, item)?;
					validate_unsigned = Some(v);
				},
				Some(NobleAttr::TypeValue(span)) =>
					type_values.push(type_value::TypeValueDef::try_from(span, index, item)?),
				Some(NobleAttr::ExtraConstants(_)) => {
					extra_constants =
						Some(extra_constants::ExtraConstantsDef::try_from(index, item)?)
				},
				Some(attr) => {
					let msg = "Invalid duplicated attribute";
					return Err(syn::Error::new(attr.span(), msg));
				},
				None => (),
			}
		}

		if genesis_config.is_some() != genesis_build.is_some() {
			let msg = format!(
				"`#[noble::genesis_config]` and `#[noble::genesis_build]` attributes must be \
				either both used or both not used, instead genesis_config is {} and genesis_build \
				is {}",
				genesis_config.as_ref().map_or("unused", |_| "used"),
				genesis_build.as_ref().map_or("unused", |_| "used"),
			);
			return Err(syn::Error::new(item_span, msg));
		}

		let def = Def {
			item,
			config: config.ok_or_else(|| syn::Error::new(item_span, "Missing `#[noble::config]`"))?,
			noble_struct: noble_struct
				.ok_or_else(|| syn::Error::new(item_span, "Missing `#[noble::noble]`"))?,
			hooks: hooks
				.ok_or_else(|| syn::Error::new(item_span, "Missing `#[noble::hooks]`"))?,
			call: call.ok_or_else(|| syn::Error::new(item_span, "Missing `#[noble::call]"))?,
			extra_constants,
			genesis_config,
			genesis_build,
			validate_unsigned,
			error,
			event,
			origin,
			inherent,
			storages,
			type_values,
			fabric_system,
			fabric_support,
		};

		def.check_instance_usage()?;
		def.check_event_usage()?;

		Ok(def)
	}

	/// Check that usage of trait `Event` is consistent with the definition, i.e. it is declared
	/// and trait defines type Event, or not declared and no trait associated type.
	fn check_event_usage(&self) -> syn::Result<()> {
		match (
			self.config.has_event_type,
			self.event.is_some(),
		) {
			(true, false) => {
				let msg = "Invalid usage of Event, `Config` contains associated type `Event`, \
					but enum `Event` is not declared (i.e. no use of `#[noble::event]`). \
					Note that type `Event` in trait is reserved to work alongside noble event.";
				Err(syn::Error::new(proc_macro2::Span::call_site(), msg))
			},
			(false, true) => {
				let msg = "Invalid usage of Event, `Config` contains no associated type \
					`Event`, but enum `Event` is declared (in use of `#[noble::event]`). \
					An Event associated type must be declare on trait `Config`.";
				Err(syn::Error::new(proc_macro2::Span::call_site(), msg))
			},
			_ => Ok(())
		}
	}

	/// Check that usage of trait `Config` is consistent with the definition, i.e. it is used with
	/// instance iff it is defined with instance.
	fn check_instance_usage(&self) -> syn::Result<()> {
		let mut instances = vec![];
		instances.extend_from_slice(&self.call.instances[..]);
		instances.extend_from_slice(&self.noble_struct.instances[..]);
		instances.extend_from_slice(&self.hooks.instances[..]);
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
		if let Some(extra_constants) = &self.extra_constants {
			instances.extend_from_slice(&extra_constants.instances[..]);
		}

		let mut errors = instances.into_iter()
			.filter_map(|instances| {
				if instances.has_instance == self.config.has_instance {
					return None
				}
				let msg = if self.config.has_instance {
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

	/// Depending on if noble is instantiable:
	/// * either `T: Config`
	/// * or `T: Config<I>, I: 'static`
	pub fn type_impl_generics(&self, span: proc_macro2::Span) -> proc_macro2::TokenStream {
		if self.config.has_instance {
			quote::quote_spanned!(span => T: Config<I>, I: 'static)
		} else {
			quote::quote_spanned!(span => T: Config)
		}
	}

	/// Depending on if noble is instantiable:
	/// * either `T: Config`
	/// * or `T: Config<I>, I: 'static = ()`
	pub fn type_decl_bounded_generics(&self, span: proc_macro2::Span) -> proc_macro2::TokenStream {
		if self.config.has_instance {
			quote::quote_spanned!(span => T: Config<I>, I: 'static = ())
		} else {
			quote::quote_spanned!(span => T: Config)
		}
	}

	/// Depending on if noble is instantiable:
	/// * either `T`
	/// * or `T, I = ()`
	pub fn type_decl_generics(&self, span: proc_macro2::Span) -> proc_macro2::TokenStream {
		if self.config.has_instance {
			quote::quote_spanned!(span => T, I = ())
		} else {
			quote::quote_spanned!(span => T)
		}
	}

	/// Depending on if noble is instantiable:
	/// * either ``
	/// * or `<I>`
	/// to be used when using noble trait `Config`
	pub fn trait_use_generics(&self, span: proc_macro2::Span) -> proc_macro2::TokenStream {
		if self.config.has_instance {
			quote::quote_spanned!(span => <I>)
		} else {
			quote::quote_spanned!(span => )
		}
	}

	/// Depending on if noble is instantiable:
	/// * either `T`
	/// * or `T, I`
	pub fn type_use_generics(&self, span: proc_macro2::Span) -> proc_macro2::TokenStream {
		if self.config.has_instance {
			quote::quote_spanned!(span => T, I)
		} else {
			quote::quote_spanned!(span => T)
		}
	}
}

/// Some generic kind for type which can be not generic, or generic over config,
/// or generic over config and instance, but not generic only over instance.
pub enum GenericKind {
	None,
	Config,
	ConfigAndInstance,
}

impl GenericKind {
	/// Return Err if it is only generics over instance but not over config.
	pub fn from_gens(has_config: bool, has_instance: bool) -> Result<Self, ()> {
		match (has_config, has_instance) {
			(false, false) => Ok(GenericKind::None),
			(true, false) => Ok(GenericKind::Config),
			(true, true) => Ok(GenericKind::ConfigAndInstance),
			(false, true) => Err(()),
		}
	}

	/// Return the generic to be used when using the type.
	///
	/// Depending on its definition it can be: ``, `T` or `T, I`
	pub fn type_use_gen(&self, span: proc_macro2::Span) -> proc_macro2::TokenStream {
		match self {
			GenericKind::None => quote::quote!(),
			GenericKind::Config => quote::quote_spanned!(span => T),
			GenericKind::ConfigAndInstance => quote::quote_spanned!(span => T, I),
		}
	}

	/// Return the generic to be used in `impl<..>` when implementing on the type.
	pub fn type_impl_gen(&self, span: proc_macro2::Span) -> proc_macro2::TokenStream {
		match self {
			GenericKind::None => quote::quote!(),
			GenericKind::Config => quote::quote_spanned!(span => T: Config),
			GenericKind::ConfigAndInstance => quote::quote_spanned!(span => T: Config<I>, I: 'static),
		}
	}

	/// Return whereas the type has some generic.
	pub fn is_generic(&self) -> bool {
		match self {
			GenericKind::None => false,
			GenericKind::Config | GenericKind::ConfigAndInstance => true,
		}
	}
}

/// List of additional token to be used for parsing.
mod keyword {
	syn::custom_keyword!(origin);
	syn::custom_keyword!(call);
	syn::custom_keyword!(event);
	syn::custom_keyword!(config);
	syn::custom_keyword!(hooks);
	syn::custom_keyword!(inherent);
	syn::custom_keyword!(error);
	syn::custom_keyword!(storage);
	syn::custom_keyword!(genesis_build);
	syn::custom_keyword!(genesis_config);
	syn::custom_keyword!(validate_unsigned);
	syn::custom_keyword!(type_value);
	syn::custom_keyword!(noble);
	syn::custom_keyword!(generate_store);
	syn::custom_keyword!(Store);
	syn::custom_keyword!(extra_constants);
}

/// Parse attributes for item in noble module
/// syntax must be `noble::` (e.g. `#[noble::config]`)
enum NobleAttr {
	Config(proc_macro2::Span),
	Noble(proc_macro2::Span),
	Hooks(proc_macro2::Span),
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
	ExtraConstants(proc_macro2::Span),
}

impl NobleAttr {
	fn span(&self) -> proc_macro2::Span {
		match self {
			Self::Config(span) => *span,
			Self::Noble(span) => *span,
			Self::Hooks(span) => *span,
			Self::Call(span) => *span,
			Self::Error(span) => *span,
			Self::Event(span) => *span,
			Self::Origin(span) => *span,
			Self::Inherent(span) => *span,
			Self::Storage(span) => *span,
			Self::GenesisConfig(span) => *span,
			Self::GenesisBuild(span) => *span,
			Self::ValidateUnsigned(span) => *span,
			Self::TypeValue(span) => *span,
			Self::ExtraConstants(span) => *span,
		}
	}
}

impl syn::parse::Parse for NobleAttr {
	fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
		input.parse::<syn::Token![#]>()?;
		let content;
		syn::bracketed!(content in input);
		content.parse::<keyword::noble>()?;
		content.parse::<syn::Token![::]>()?;

		let lookahead = content.lookahead1();
		if lookahead.peek(keyword::config) {
			Ok(NobleAttr::Config(content.parse::<keyword::config>()?.span()))
		} else if lookahead.peek(keyword::noble) {
			Ok(NobleAttr::Noble(content.parse::<keyword::noble>()?.span()))
		} else if lookahead.peek(keyword::hooks) {
			Ok(NobleAttr::Hooks(content.parse::<keyword::hooks>()?.span()))
		} else if lookahead.peek(keyword::call) {
			Ok(NobleAttr::Call(content.parse::<keyword::call>()?.span()))
		} else if lookahead.peek(keyword::error) {
			Ok(NobleAttr::Error(content.parse::<keyword::error>()?.span()))
		} else if lookahead.peek(keyword::event) {
			Ok(NobleAttr::Event(content.parse::<keyword::event>()?.span()))
		} else if lookahead.peek(keyword::origin) {
			Ok(NobleAttr::Origin(content.parse::<keyword::origin>()?.span()))
		} else if lookahead.peek(keyword::inherent) {
			Ok(NobleAttr::Inherent(content.parse::<keyword::inherent>()?.span()))
		} else if lookahead.peek(keyword::storage) {
			Ok(NobleAttr::Storage(content.parse::<keyword::storage>()?.span()))
		} else if lookahead.peek(keyword::genesis_config) {
			Ok(NobleAttr::GenesisConfig(content.parse::<keyword::genesis_config>()?.span()))
		} else if lookahead.peek(keyword::genesis_build) {
			Ok(NobleAttr::GenesisBuild(content.parse::<keyword::genesis_build>()?.span()))
		} else if lookahead.peek(keyword::validate_unsigned) {
			Ok(NobleAttr::ValidateUnsigned(content.parse::<keyword::validate_unsigned>()?.span()))
		} else if lookahead.peek(keyword::type_value) {
			Ok(NobleAttr::TypeValue(content.parse::<keyword::type_value>()?.span()))
		} else if lookahead.peek(keyword::extra_constants) {
			Ok(NobleAttr::ExtraConstants(content.parse::<keyword::extra_constants>()?.span()))
		} else {
			Err(lookahead.error())
		}
	}
}
