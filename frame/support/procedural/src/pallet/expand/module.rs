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

/// * Add derive Eq, PartialEq, Debug and Clone on Module
/// * if event is defined, implement deposit_event on module.
pub fn expand_module(def: &mut Def) -> proc_macro2::TokenStream {
	let scrate = &def.scrate();

	let module_item = {
		let item = &mut def.item.content.as_mut().expect("Checked by def").1[def.module.index];
		if let syn::Item::Struct(item) = item {
			item
		} else {
			unreachable!("Checked by module parser")
		}
	};

	module_item.attrs.push(syn::parse_quote!(
		#[derive(
			#scrate::CloneNoBound,
			#scrate::EqNoBound,
			#scrate::PartialEqNoBound,
			#scrate::DebugStripped,
		)]
	));

	if let Some(fn_deposit_event_span) = def.module.generate_fn_deposit_event {
		let event = def.event.as_ref().expect("Checked by parser");
		let event_use_gen = &event.event_use_gen();
		let trait_use_gen = &def.trait_use_generics();
		let type_impl_gen = &def.type_impl_generics();
		let type_use_gen = &def.type_use_generics();

		quote::quote_spanned!(fn_deposit_event_span =>
			impl<#type_impl_gen> Module<#type_use_gen> {
				fn deposit_event(event: Event<#event_use_gen>) {
					let event = <
						<T as Trait#trait_use_gen>::Event as
						From<Event<#event_use_gen>>
					>::from(event);

					let event = <
						<T as Trait#trait_use_gen>::Event as
						Into<<T as frame_system::Trait>::Event>
					>::into(event);

					<frame_system::Module<T>>::deposit_event(event)
				}
			}
		)
	} else {
		Default::default()
	}
}
