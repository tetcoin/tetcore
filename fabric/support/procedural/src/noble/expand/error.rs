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

/// * impl various trait on Error
/// * impl ModuleErrorMetadata for Error
pub fn expand_error(def: &mut Def) -> proc_macro2::TokenStream {
	let error = if let Some(error) = &def.error {
		error
	} else {
		return Default::default()
	};

	let error_ident = &error.error;
	let fabric_support = &def.fabric_support;
	let fabric_system = &def.fabric_system;
	let type_impl_gen = &def.type_impl_generics(error.attr_span);
	let type_use_gen = &def.type_use_generics(error.attr_span);
	let config_where_clause = &def.config.where_clause;

	let phantom_variant: syn::Variant = syn::parse_quote!(
		#[doc(hidden)]
		__Ignore(
			#fabric_support::tetcore_std::marker::PhantomData<(#type_use_gen)>,
			#fabric_support::Never,
		)
	);

	let as_u8_matches = error.variants.iter().enumerate()
		.map(|(i, (variant, _))| {
			quote::quote_spanned!(error.attr_span => Self::#variant => #i as u8,)
		});

	let as_str_matches = error.variants.iter()
		.map(|(variant, _)| {
			let variant_str = format!("{}", variant);
			quote::quote_spanned!(error.attr_span => Self::#variant => #variant_str,)
		});

	let metadata = error.variants.iter()
		.map(|(variant, doc)| {
			let variant_str = format!("{}", variant);
			quote::quote_spanned!(error.attr_span =>
				#fabric_support::error::ErrorMetadata {
					name: #fabric_support::error::DecodeDifferent::Encode(#variant_str),
					documentation: #fabric_support::error::DecodeDifferent::Encode(&[ #( #doc, )* ]),
				},
			)
		});

	let error_item = {
		let item = &mut def.item.content.as_mut().expect("Checked by def parser").1[error.index];
		if let syn::Item::Enum(item) = item {
			item
		} else {
			unreachable!("Checked by error parser")
		}
	};

	error_item.variants.insert(0, phantom_variant);

	quote::quote_spanned!(error.attr_span =>
		impl<#type_impl_gen> #fabric_support::tetcore_std::fmt::Debug for #error_ident<#type_use_gen>
			#config_where_clause
		{
			fn fmt(&self, f: &mut #fabric_support::tetcore_std::fmt::Formatter<'_>)
				-> #fabric_support::tetcore_std::fmt::Result
			{
				f.write_str(self.as_str())
			}
		}

		impl<#type_impl_gen> #error_ident<#type_use_gen> #config_where_clause {
			pub fn as_u8(&self) -> u8 {
				match &self {
					Self::__Ignore(_, _) => unreachable!("`__Ignore` can never be constructed"),
					#( #as_u8_matches )*
				}
			}

			pub fn as_str(&self) -> &'static str {
				match &self {
					Self::__Ignore(_, _) => unreachable!("`__Ignore` can never be constructed"),
					#( #as_str_matches )*
				}
			}
		}

		impl<#type_impl_gen> From<#error_ident<#type_use_gen>> for &'static str
			#config_where_clause
		{
			fn from(err: #error_ident<#type_use_gen>) -> &'static str {
				err.as_str()
			}
		}

		impl<#type_impl_gen> From<#error_ident<#type_use_gen>>
			for #fabric_support::tp_runtime::DispatchError
			#config_where_clause
		{
			fn from(err: #error_ident<#type_use_gen>) -> Self {
				let index = <
					<T as #fabric_system::Config>::NobleInfo
					as #fabric_support::traits::NobleInfo
				>::index::<Noble<#type_use_gen>>()
					.expect("Every active module has an index in the runtime; qed") as u8;

				#fabric_support::tp_runtime::DispatchError::Module {
					index,
					error: err.as_u8(),
					message: Some(err.as_str()),
				}
			}
		}

		impl<#type_impl_gen> #fabric_support::error::ModuleErrorMetadata
			for #error_ident<#type_use_gen>
			#config_where_clause
		{
			fn metadata() -> &'static [#fabric_support::error::ErrorMetadata] {
				&[ #( #metadata )* ]
			}
		}
	)
}
