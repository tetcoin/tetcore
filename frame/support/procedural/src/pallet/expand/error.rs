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

/// * impl Debug for Error
/// * impl as_u8 and as_str for Error
/// * impl `From<Error>` for static str
/// * impl `From<Error>` for DispatchError
/// * impl ModuleErrorMetadata for Error
pub fn expand_error(def: &mut Def) -> proc_macro2::TokenStream {
	let error = if let Some(error) = &def.error {
		error
	} else {
		return Default::default()
	};

	let error_ident = &error.error;
	let scrate = &def.scrate();
	let type_impl_gen = &def.type_impl_generics();
	let type_impl_static_gen = &def.type_impl_static_generics();
	let type_use_gen = &def.type_use_generics();

	let phantom_variant: syn::Variant = syn::parse_quote!(
		#[doc(hidden)]
		__Ignore(
			#scrate::sp_std::marker::PhantomData<(#type_use_gen)>,
			#scrate::Never,
		)
	);

	let as_u8_matches = error.variants.iter().enumerate()
		.map(|(i, (variant, _))| quote::quote!(Self::#variant => #i as u8,));

	let as_str_matches = error.variants.iter()
		.map(|(variant, _)| {
			let variant_str = format!("{}", variant);
			quote::quote!(Self::#variant => #variant_str,)
		});

	let metadata = error.variants.iter()
		.map(|(variant, doc)| {
			let variant_str = format!("{}", variant);
			quote::quote!(
				#scrate::error::ErrorMetadata {
					name: #scrate::error::DecodeDifferent::Encode(#variant_str),
					documentation: #scrate::error::DecodeDifferent::Encode(&[ #( #doc, )* ]),
				},
			)
		});

	let error_item = {
		let item = &mut def.item.content.as_mut().expect("Checked by def parser").1[error.index];
		if let syn::Item::Enum(item) = item {
			item
		} else {
			unreachable!("Checked by event parser")
		}
	};

	error_item.variants.insert(0, phantom_variant);

	quote::quote!(
		impl<#type_impl_gen> #scrate::sp_std::fmt::Debug for #error_ident<#type_use_gen> {
			fn fmt(&self, f: &mut #scrate::sp_std::fmt::Formatter<'_>)
				-> #scrate::sp_std::fmt::Result
			{
				f.write_str(self.as_str())
			}
		}

		impl<#type_impl_gen> #error_ident<#type_use_gen> {
			fn as_u8(&self) -> u8 {
				match &self {
					Self::__Ignore(_, _) => unreachable!("`__Ignore` can never be constructed"),
					#( #as_u8_matches )*
				}
			}

			fn as_str(&self) -> &'static str {
				match &self {
					Self::__Ignore(_, _) => unreachable!("`__Ignore` can never be constructed"),
					#( #as_str_matches )*
				}
			}
		}

		impl<#type_impl_gen> From<#error_ident<#type_use_gen>> for &'static str {
			fn from(err: #error_ident<#type_use_gen>) -> &'static str {
				err.as_str()
			}
		}

		impl<#type_impl_static_gen> From<#error_ident<#type_use_gen>>
			for #scrate::sp_runtime::DispatchError
		{
			fn from(err: #error_ident<#type_use_gen>) -> Self {
				let index = <
					<T as frame_system::Trait>::ModuleToIndex
					as #scrate::traits::ModuleToIndex
				>::module_to_index::<Module<#type_use_gen>>()
					.expect("Every active module has an index in the runtime; qed") as u8;

				#scrate::sp_runtime::DispatchError::Module {
					index,
					error: err.as_u8(),
					message: Some(err.as_str()),
				}
			}
		}

		impl<#type_impl_gen> #scrate::error::ModuleErrorMetadata for #error_ident<#type_use_gen> {
			fn metadata() -> &'static [#scrate::error::ErrorMetadata] {
				&[ #( #metadata )* ]
			}
		}
	)
}

