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

/// * implement the individual traits using the Hooks trait
pub fn expand_hooks(def: &mut Def) -> proc_macro2::TokenStream {
	let fabric_support = &def.fabric_support;
	let type_impl_gen = &def.type_impl_generics(def.hooks.attr_span);
	let type_use_gen = &def.type_use_generics(def.hooks.attr_span);
	let noble_ident = &def.noble_struct.noble;
	let where_clause = &def.hooks.where_clause;
	let fabric_system = &def.fabric_system;

	quote::quote_spanned!(def.hooks.attr_span =>
		impl<#type_impl_gen>
			#fabric_support::traits::OnFinalize<<T as #fabric_system::Config>::BlockNumber>
			for #noble_ident<#type_use_gen> #where_clause
		{
			fn on_finalize(n: <T as #fabric_system::Config>::BlockNumber) {
				<
					Self as #fabric_support::traits::Hooks<
						<T as #fabric_system::Config>::BlockNumber
					>
				>::on_finalize(n)
			}
		}

		impl<#type_impl_gen>
			#fabric_support::traits::OnInitialize<<T as #fabric_system::Config>::BlockNumber>
			for #noble_ident<#type_use_gen> #where_clause
		{
			fn on_initialize(
				n: <T as #fabric_system::Config>::BlockNumber
			) -> #fabric_support::weights::Weight {
				<
					Self as #fabric_support::traits::Hooks<
						<T as #fabric_system::Config>::BlockNumber
					>
				>::on_initialize(n)
			}
		}

		impl<#type_impl_gen>
			#fabric_support::traits::OnRuntimeUpgrade
			for #noble_ident<#type_use_gen> #where_clause
		{
			fn on_runtime_upgrade() -> #fabric_support::weights::Weight {
				let result = <
					Self as #fabric_support::traits::Hooks<
						<T as #fabric_system::Config>::BlockNumber
					>
				>::on_runtime_upgrade();

				#fabric_support::crate_to_noble_version!()
					.put_into_storage::<<T as #fabric_system::Config>::NobleInfo, Self>();

				let additional_write = <
					<T as #fabric_system::Config>::DbWeight as #fabric_support::traits::Get<_>
				>::get().writes(1);

				result.saturating_add(additional_write)
			}
		}

		impl<#type_impl_gen>
			#fabric_support::traits::OffchainWorker<<T as #fabric_system::Config>::BlockNumber>
			for #noble_ident<#type_use_gen> #where_clause
		{
			fn offchain_worker(n: <T as #fabric_system::Config>::BlockNumber) {
				<
					Self as #fabric_support::traits::Hooks<
						<T as #fabric_system::Config>::BlockNumber
					>
				>::offchain_worker(n)
			}
		}

		impl<#type_impl_gen>
			#fabric_support::traits::IntegrityTest
			for #noble_ident<#type_use_gen> #where_clause
		{
			fn integrity_test() {
				<
					Self as #fabric_support::traits::Hooks<
						<T as #fabric_system::Config>::BlockNumber
					>
				>::integrity_test()
			}
		}
	)
}
