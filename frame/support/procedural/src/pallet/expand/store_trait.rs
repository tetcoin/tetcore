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

/// * generate Store trait
/// * implement Store trait for module
pub fn expand_store_trait(def: &mut Def) -> proc_macro2::TokenStream {
	let type_impl_static_gen = &def.type_impl_static_generics();
	let type_use_gen = &def.type_use_generics();
	let module_ident = &def.module.module;

	let pub_storages = def.storages.iter()
		.filter_map(|storage| if let syn::Visibility::Public(_) = storage.vis {
			let storage_generics = match (storage.has_trait, storage.has_instance) {
				(true, true) => quote::quote!(T, I),
				(true, false) => quote::quote!(T),
				(false, true) => quote::quote!(I),
				(false, false) => quote::quote!(),
			};
			Some((storage.ident.clone(), storage_generics))
		} else {
			None
		});

	let pub_storage_names = pub_storages.clone().map(|s| s.0).collect::<Vec<_>>();
	let pub_storage_generics = pub_storages.map(|s| s.1);

	quote::quote!(
		trait Store {
			#(
				type #pub_storage_names;
			)*
		}
		impl<#type_impl_static_gen> Store for #module_ident<#type_use_gen> {
			#(
				type #pub_storage_names = #pub_storage_names<#pub_storage_generics>;
			)*
		}
	)
}
