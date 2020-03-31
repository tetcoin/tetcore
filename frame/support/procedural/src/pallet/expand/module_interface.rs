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

/// * implement the individual traits using the ModuleInterface trait
pub fn expand_module_interface(def: &mut Def) -> proc_macro2::TokenStream {
	let scrate = &def.scrate();
	let type_impl_gen = &def.type_impl_generics();
	let type_use_gen = &def.type_use_generics();
	let module_ident = &def.module.module;

	quote::quote!(
		impl<#type_impl_gen>
			#scrate::traits::OnFinalize<<T as frame_system::Trait>::BlockNumber>
			for #module_ident<#type_use_gen>
		{
			fn on_finalize(n: <T as frame_system::Trait>::BlockNumber) {
				<
					Self as #scrate::traits::ModuleInterface<
						<T as frame_system::Trait>::BlockNumber
					>
				>::on_finalize(n)
			}
		}

		impl<#type_impl_gen>
			#scrate::traits::OnInitialize<<T as frame_system::Trait>::BlockNumber>
			for #module_ident<#type_use_gen>
		{
			fn on_initialize(
				n: <T as frame_system::Trait>::BlockNumber
			) -> #scrate::weights::Weight {
				<
					Self as #scrate::traits::ModuleInterface<
						<T as frame_system::Trait>::BlockNumber
					>
				>::on_initialize(n)
			}
		}

		impl<#type_impl_gen>
			#scrate::traits::OnRuntimeUpgrade
			for #module_ident<#type_use_gen>
		{
			fn on_runtime_upgrade() -> #scrate::weights::Weight {
				<
					Self as #scrate::traits::ModuleInterface<
						<T as frame_system::Trait>::BlockNumber
					>
				>::on_runtime_upgrade()
			}
		}

		impl<#type_impl_gen>
			#scrate::traits::OffchainWorker<<T as frame_system::Trait>::BlockNumber>
			for #module_ident<#type_use_gen>
		{
			fn offchain_worker(n: <T as frame_system::Trait>::BlockNumber) {
				<
					Self as #scrate::traits::ModuleInterface<
						<T as frame_system::Trait>::BlockNumber
					>
				>::offchain_worker(n)
			}
		}
	)
}
