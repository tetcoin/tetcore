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
use frame_support_procedural_tools::clean_type_string;

/// * Impl fn module_constant_metadata for module.
pub fn expand_trait_(def: &mut Def) -> proc_macro2::TokenStream {
	let scrate = &def.scrate();
	let type_impl_gen = &def.type_impl_generics();
	let type_impl_static_gen = &def.type_impl_static_generics();
	let type_decl_gen = &def.type_decl_generics();
	let type_use_gen = &def.type_use_generics();
	let module_ident = &def.module.module;

	let consts = def.trait_.consts_metadata.iter()
		.map(|const_| {
			let type_ = &const_.type_;
			let type_str = clean_type_string(&quote::quote!(#type_).to_string());
			let ident = &const_.ident;
			let ident_str = format!("{}", ident);
			let doc = const_.doc.clone().into_iter();
			let default_byte_getter = syn::Ident::new(
				&format!("{}DefaultByteGetter", ident),
				ident.span()
			);

			quote::quote!({
				#[allow(non_upper_case_types)]
				#[allow(non_camel_case_types)]
				struct #default_byte_getter<#type_decl_gen>(
					#scrate::sp_std::marker::PhantomData<(#type_use_gen)>
				);

				impl<#type_impl_gen> #scrate::dispatch::DefaultByte for
					#default_byte_getter<#type_use_gen>
				{
					fn default_byte(&self) -> #scrate::sp_std::vec::Vec<u8> {
						let value = <T::#ident as #scrate::traits::Get<#type_>>::get();
						#scrate::codec::Encode::encode(&value)
					}
				}
				// TODO TODO: maybe use the struct in frame_support::storage::type

				unsafe impl<#type_impl_gen> Send for #default_byte_getter<#type_use_gen> {}
				unsafe impl<#type_impl_gen> Sync for #default_byte_getter<#type_use_gen> {}

				#scrate::dispatch::ModuleConstantMetadata {
					name: #scrate::dispatch::DecodeDifferent::Encode(#ident_str),
					ty: #scrate::dispatch::DecodeDifferent::Encode(#type_str),
					value: #scrate::dispatch::DecodeDifferent::Encode(
						#scrate::dispatch::DefaultByteGetter(
							&#default_byte_getter::<#type_use_gen>(
								#scrate::sp_std::marker::PhantomData
							)
						)
					),
					documentation: #scrate::dispatch::DecodeDifferent::Encode(
						&[ #( #doc )* ]
					),
				}
			})
		});

	quote::quote!(
		impl<#type_impl_static_gen> #module_ident<#type_use_gen> {

			#[doc(hidden)]
			pub fn module_constants_metadata()
				-> &'static [#scrate::dispatch::ModuleConstantMetadata]
			{
				&[ #( #consts )* ]
			}
		}
	)
}
