// SPDX-FileCopyrightText: 2026, Shaun Reed <shaunrd0@gmail.com>
//
// SPDX-License-Identifier: GNU General Public License v3.0 or later

use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemStruct, parse_macro_input};

#[proc_macro_derive(Loggable)]
pub fn loggable(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    let struct_name = &input.ident;
    let generics = &input.generics;

    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();
    let struct_name_str = struct_name.to_string();
    let expanded = quote! {
        impl #impl_generics libclide::log::Loggable for #struct_name #type_generics #where_clause {
            const ID: &'static str = #struct_name_str;
        }
    };

    TokenStream::from(expanded)
}
