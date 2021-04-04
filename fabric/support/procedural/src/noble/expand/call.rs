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

use crate::noble::Def;
use fabric_support_procedural_tools::clean_type_string;
use syn::spanned::Spanned;

/// * Generate enum call and implement various trait on it.
/// * Implement Callable and call_function on `Noble`
pub fn expand_call(def: &mut Def) -> proc_macro2::TokenStream {
	let fabric_support = &def.fabric_support;
	let fabric_system = &def.fabric_system;
	let type_impl_gen = &def.type_impl_generics(def.call.attr_span);
	let type_decl_bounded_gen = &def.type_decl_bounded_generics(def.call.attr_span);
	let type_use_gen = &def.type_use_generics(def.call.attr_span);
	let call_ident = syn::Ident::new("Call", def.call.attr_span);
	let noble_ident = &def.noble_struct.noble;
	let where_clause = &def.call.where_clause;

	let fn_name = def.call.methods.iter().map(|method| &method.name).collect::<Vec<_>>();

	let fn_weight = def.call.methods.iter().map(|method| &method.weight);

	let fn_doc = def.call.methods.iter().map(|method| &method.docs).collect::<Vec<_>>();

	let args_name = def.call.methods.iter()
		.map(|method| method.args.iter().map(|(_, name, _)| name.clone()).collect::<Vec<_>>())
		.collect::<Vec<_>>();

	let args_type = def.call.methods.iter()
		.map(|method| method.args.iter().map(|(_, _, type_)| type_.clone()).collect::<Vec<_>>())
		.collect::<Vec<_>>();

	let args_compact_attr = def.call.methods.iter().map(|method| {
		method.args.iter()
			.map(|(is_compact, _, type_)| {
				if *is_compact {
					quote::quote_spanned!(type_.span() => #[codec(compact)] )
				} else {
					quote::quote!()
				}
			})
			.collect::<Vec<_>>()
	});

	let args_metadata_type = def.call.methods.iter().map(|method| {
		method.args.iter()
			.map(|(is_compact, _, type_)| {
				let final_type = if *is_compact {
					quote::quote_spanned!(type_.span() => Compact<#type_>)
				} else {
					quote::quote!(#type_)
				};
				clean_type_string(&final_type.to_string())
			})
			.collect::<Vec<_>>()
	});

	quote::quote_spanned!(def.call.attr_span =>
		#[derive(
			#fabric_support::RuntimeDebugNoBound,
			#fabric_support::CloneNoBound,
			#fabric_support::EqNoBound,
			#fabric_support::PartialEqNoBound,
			#fabric_support::codec::Encode,
			#fabric_support::codec::Decode,
		)]
		#[allow(non_camel_case_types)]
		pub enum #call_ident<#type_decl_bounded_gen> #where_clause {
			#[doc(hidden)]
			#[codec(skip)]
			__Ignore(
				#fabric_support::tetcore_std::marker::PhantomData<(#type_use_gen,)>,
				#fabric_support::Never,
			),
			#( #fn_name( #( #args_compact_attr #args_type ),* ), )*
		}

		impl<#type_impl_gen> #fabric_support::dispatch::GetDispatchInfo
			for #call_ident<#type_use_gen>
			#where_clause
		{
			fn get_dispatch_info(&self) -> #fabric_support::dispatch::DispatchInfo {
				match *self {
					#(
						Self::#fn_name ( #( ref #args_name, )* ) => {
							let base_weight = #fn_weight;

							let weight = <
								dyn #fabric_support::dispatch::WeighData<( #( & #args_type, )* )>
							>::weigh_data(&base_weight, ( #( #args_name, )* ));

							let class = <
								dyn #fabric_support::dispatch::ClassifyDispatch<
									( #( & #args_type, )* )
								>
							>::classify_dispatch(&base_weight, ( #( #args_name, )* ));

							let pays_fee = <
								dyn #fabric_support::dispatch::PaysFee<( #( & #args_type, )* )>
							>::pays_fee(&base_weight, ( #( #args_name, )* ));

							#fabric_support::dispatch::DispatchInfo {
								weight,
								class,
								pays_fee,
							}
						},
					)*
					Self::__Ignore(_, _) => unreachable!("__Ignore cannot be used"),
				}
			}
		}

		impl<#type_impl_gen> #fabric_support::dispatch::GetCallName for #call_ident<#type_use_gen>
			#where_clause
		{
			fn get_call_name(&self) -> &'static str {
				match *self {
					#( Self::#fn_name(..) => stringify!(#fn_name), )*
					Self::__Ignore(_, _) => unreachable!("__PhantomItem cannot be used."),
				}
			}

			fn get_call_names() -> &'static [&'static str] {
				&[ #( stringify!(#fn_name), )* ]
			}
		}

		impl<#type_impl_gen> #fabric_support::traits::UnfilteredDispatchable
			for #call_ident<#type_use_gen>
			#where_clause
		{
			type Origin = #fabric_system::noble_prelude::OriginFor<T>;
			fn dispatch_bypass_filter(
				self,
				origin: Self::Origin
			) -> #fabric_support::dispatch::DispatchResultWithPostInfo {
				match self {
					#(
						Self::#fn_name( #( #args_name, )* ) =>
							<#noble_ident<#type_use_gen>>::#fn_name(origin, #( #args_name, )* )
								.map(Into::into).map_err(Into::into),
					)*
					Self::__Ignore(_, _) => {
						let _ = origin; // Use origin for empty Call enum
						unreachable!("__PhantomItem cannot be used.");
					},
				}
			}
		}

		impl<#type_impl_gen> #fabric_support::dispatch::Callable<T> for #noble_ident<#type_use_gen>
			#where_clause
		{
			type Call = #call_ident<#type_use_gen>;
		}

		impl<#type_impl_gen> #noble_ident<#type_use_gen> #where_clause {
			#[doc(hidden)]
			pub fn call_functions() -> &'static [#fabric_support::dispatch::FunctionMetadata] {
				&[ #(
					#fabric_support::dispatch::FunctionMetadata {
						name: #fabric_support::dispatch::DecodeDifferent::Encode(
							stringify!(#fn_name)
						),
						arguments: #fabric_support::dispatch::DecodeDifferent::Encode(
							&[ #(
								#fabric_support::dispatch::FunctionArgumentMetadata {
									name: #fabric_support::dispatch::DecodeDifferent::Encode(
										stringify!(#args_name)
									),
									ty: #fabric_support::dispatch::DecodeDifferent::Encode(
										#args_metadata_type
									),
								},
							)* ]
						),
						documentation: #fabric_support::dispatch::DecodeDifferent::Encode(
							&[ #( #fn_doc ),* ]
						),
					},
				)* ]
			}
		}
	)
}
