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

/// * create Call enum
/// * impl GetDispatchInfo for Call
/// * impl GetCallName for Call
/// * impl UnfilteredDispatchable for Call
/// * impl Callable for Module
/// * impl call_functions for Module (metadata)
pub fn expand_call(def: &mut Def) -> proc_macro2::TokenStream {
	let scrate = &def.scrate();
	let type_impl_gen = &def.type_impl_generics();
	let type_decl_gen = &def.type_decl_generics();
	let type_use_gen = &def.type_use_generics();
	let call_ident = &def.call.call;
	let module_ident = &def.module.module;
	let where_clause = &def.call.where_clause;

	let fn_ = def.call.methods.iter().map(|method| &method.fn_).collect::<Vec<_>>();

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
			.map(|(is_compact, _, _)| {
				if *is_compact {
					quote::quote!( #[codec(compact)] )
				} else {
					quote::quote!()
				}
			})
			.collect::<Vec<_>>()
	});

	let args_metadata_type = def.call.methods.iter().map(|method| {
		method.args.iter()
			.map(|(is_compact, _, type_)| {
				if *is_compact {
					format!("Compact<{:?}>", type_)
				} else {
					format!("{:?}", type_)
				}
			})
			.collect::<Vec<_>>()
	});

	quote::quote!(
		#[derive(
			#scrate::codec::Encode,
			#scrate::codec::Decode,
			#scrate::CloneNoBound,
			#scrate::EqNoBound,
			#scrate::PartialEqNoBound,
		)]
		#[cfg_attr(feature = "std", derive(#scrate::DebugNoBound))]
		#[cfg_attr(not(feature = "std"), derive(#scrate::DebugStripped))]
		#[allow(non_camel_case_types)]
		pub enum #call_ident<#type_decl_gen> {
			#[doc(hidden)]
			#[codec(skip)]
			__Ignore(
				#scrate::sp_std::marker::PhantomData<(#type_use_gen)>,
				#scrate::Never,
			),
			#( #fn_( #( #args_compact_attr #args_type )* ), )*
		}

		impl<#type_impl_gen> #scrate::dispatch::GetDispatchInfo for #call_ident<#type_use_gen>
			#where_clause
		{
			fn get_dispatch_info(&self) -> #scrate::dispatch::DispatchInfo {
				match *self {
					#(
						Self::#fn_ ( #( ref #args_name, )* ) => {
							let base_weight = #fn_weight;

							let weight = <
								dyn #scrate::dispatch::WeighData<( #( & #args_type, )* )>
							>::weigh_data(&base_weight, ( #( #args_name, )* ));

							let class = <
								dyn #scrate::dispatch::ClassifyDispatch<( #( & #args_type, )* )>
							>::classify_dispatch(&base_weight, ( #( #args_name, )* ));

							let pays_fee = <
								dyn #scrate::dispatch::PaysFee<( #( & #args_type, )* )>
							>::pays_fee(&base_weight, ( #( #args_name, )* ));

							#scrate::dispatch::DispatchInfo {
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

		impl<#type_impl_gen> #scrate::dispatch::GetCallName for #call_ident<#type_use_gen>
			#where_clause
		{
			fn get_call_name(&self) -> &'static str {
				match *self {
					#( Self::#fn_(..) => stringify!(#fn_), )*
					Self::__Ignore(_, _) => unreachable!("__PhantomItem cannot be used."),
				}
			}

			fn get_call_names() -> &'static [&'static str] {
				&[ #( stringify!(#fn_), )* ]
			}
		}

		impl<#type_impl_gen> #scrate::traits::UnfilteredDispatchable for #call_ident<#type_use_gen>
			#where_clause
		{
			type Origin = OriginFor<T>;
			fn dispatch_bypass_filter(
				self,
				origin: Self::Origin
			) -> #scrate::dispatch::DispatchResultWithPostInfo {
				match self {
					#(
						Self::#fn_( #( #args_name, )* ) =>
							<#module_ident<#type_use_gen>>::#fn_(origin, #( #args_name, )* )
								.map(Into::into).map_err(Into::into),
					)*
					Self::__Ignore(_, _) => unreachable!("__PhantomItem cannot be used."),
				}
			}
		}

		impl<#type_impl_gen> #scrate::dispatch::Callable<T> for #module_ident<#type_use_gen>
			#where_clause
		{
			type Call = #call_ident<#type_use_gen>;
		}

		impl<#type_impl_gen> #module_ident<#type_use_gen> #where_clause {
			#[doc(hidden)]
			pub fn call_functions() -> &'static [#scrate::dispatch::FunctionMetadata] {
				&[ #(
					#scrate::dispatch::FunctionMetadata {
						name: #scrate::dispatch::DecodeDifferent::Encode(stringify!(#fn_)),
						arguments: #scrate::dispatch::DecodeDifferent::Encode(
							&[ #(
								#scrate::dispatch::FunctionArgumentMetadata {
									name: #scrate::dispatch::DecodeDifferent::Encode(
										stringify!(#args_name)
									),
									ty: #scrate::dispatch::DecodeDifferent::Encode(
										#args_metadata_type
									),
								},
							)* ]
						),
						documentation: #scrate::dispatch::DecodeDifferent::Encode(
							&[ #( #fn_doc ),* ]
						),
					},
				)* ]
			}
		}
	)
}

