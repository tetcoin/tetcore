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

use crate::pallet::Def;
use proc_macro2::Span;

/// for instantiable pallet implement default instance and instance from 0 to 16,
/// for non instantiable pallet implement the inherent instance.
pub fn expand_instances(def: &mut Def) -> proc_macro2::TokenStream {
	let scrate = &def.scrate();
	let pallet_prefix = def.name.to_string();

	let inherent_ident = syn::Ident::new(crate::INHERENT_INSTANCE_NAME, Span::call_site());
	let inherent_attrs = quote::quote!(
		/// Hidden instance generated to be internally used when module is used without
		/// instance.
		#[doc(hidden)]
	);

	let mut instances = vec![];
	let mut maybe_inherent_alias = None;

	if def.trait_.has_instance {
		for i in 0..16 {
			let ident = syn::Ident::new(&format!("Instance{}", i), Span::call_site());
			let prefix = format!("Instance{}{}", i, pallet_prefix);
			let doc = quote::quote!(
				/// Generated module instance
			);
			instances.push((ident, prefix, doc));
		}

		let default_ident = syn::Ident::new("DefaultInstance", Span::call_site());
		let default_attrs = quote::quote!(
			/// Generated module default instance
		);
		instances.push((default_ident, pallet_prefix.clone(), default_attrs));
	}

	if def.trait_.has_instance {
		maybe_inherent_alias = Some(quote::quote!(
			#inherent_attrs
			pub type #inherent_ident = DefaultInstance;
		));
	} else {
		instances.push((inherent_ident, pallet_prefix, inherent_attrs));
	}

	let instance_names = instances.iter().map(|i| &i.0);
	let instance_prefixes = instances.iter().map(|i| &i.1);
	let instance_attrs = instances.iter().map(|i| &i.2);

	quote::quote!(
		#maybe_inherent_alias

		#(
			// Those trait are derived because of wrong bounds for generics
			#[derive(
				Clone, Eq, PartialEq,
				#scrate::codec::Encode,
				#scrate::codec::Decode,
				#scrate::RuntimeDebug,
			)]
			#instance_attrs
			pub struct #instance_names;
			impl #scrate::traits::Instance for #instance_names {
				const PREFIX: &'static str = #instance_prefixes;
			}
		)*
	)
}
