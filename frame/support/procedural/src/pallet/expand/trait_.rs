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
use core::iter::FromIterator;
use quote::ToTokens;

/// Replace ident `Self` by `T`
fn replace_self_by_t(input: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
	let output = input.into_iter()
		.map(|token_tree| match token_tree {
			proc_macro2::TokenTree::Group(group) =>
				proc_macro2::Group::new(
					group.delimiter(),
					replace_self_by_t(group.stream())
				).into(),
			proc_macro2::TokenTree::Ident(ident) if ident == "Self" =>
				proc_macro2::Ident::new("T", ident.span()).into(),
			other @ _ => other
		});

	proc_macro2::TokenStream::from_iter(output)
}

/// * Impl fn module_constant_metadata for module.
pub fn expand_trait_(def: &mut Def) -> proc_macro2::TokenStream {
	let frame_support = &def.frame_support;
	let type_impl_gen = &def.type_impl_generics();
	let type_decl_gen = &def.type_decl_generics();
	let type_use_gen = &def.type_use_generics();
	let module_ident = &def.module.module;

	let consts = def.trait_.consts_metadata.iter()
		.map(|const_| {
			let const_type = replace_self_by_t(const_.type_.to_token_stream());
			let const_type_str = clean_type_string(&const_type.to_string());
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
					#frame_support::sp_std::marker::PhantomData<(#type_use_gen)>
				);

				impl<#type_impl_gen> #frame_support::dispatch::DefaultByte for
					#default_byte_getter<#type_use_gen>
				{
					fn default_byte(&self) -> #frame_support::sp_std::vec::Vec<u8> {
						let value = <T::#ident as #frame_support::traits::Get<#const_type>>::get();
						#frame_support::codec::Encode::encode(&value)
					}
				}

				unsafe impl<#type_impl_gen> Send for #default_byte_getter<#type_use_gen> {}
				unsafe impl<#type_impl_gen> Sync for #default_byte_getter<#type_use_gen> {}

				#frame_support::dispatch::ModuleConstantMetadata {
					name: #frame_support::dispatch::DecodeDifferent::Encode(#ident_str),
					ty: #frame_support::dispatch::DecodeDifferent::Encode(#const_type_str),
					value: #frame_support::dispatch::DecodeDifferent::Encode(
						#frame_support::dispatch::DefaultByteGetter(
							&#default_byte_getter::<#type_use_gen>(
								#frame_support::sp_std::marker::PhantomData
							)
						)
					),
					documentation: #frame_support::dispatch::DecodeDifferent::Encode(
						&[ #( #doc ),* ]
					),
				}
			})
		});

	quote::quote!(
		impl<#type_impl_gen> #module_ident<#type_use_gen> {

			#[doc(hidden)]
			pub fn module_constants_metadata()
				-> &'static [#frame_support::dispatch::ModuleConstantMetadata]
			{
				&[ #( #consts ),* ]
			}
		}
	)
}
