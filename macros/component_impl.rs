//! Implementation of the #[component] attribute

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

/// Implementation of the #[component] attribute
pub fn component(input: TokenStream) -> TokenStream {
    let func = parse_macro_input!(input as syn::ItemFn);

    let name = func.sig.ident;
    let vis = func.vis;

    quote! {
        #[derive(Debug)]
        #vis struct #name {

        }

        impl rweb::prelude::Component for #name {
            fn build(&self) -> rweb::build::BuildCache {
                todo!()
            }

            fn init_sigs(&mut self) {
                // Signals aren't yet supported
            }
        }

        impl Default for #name {
            fn default() -> Self {
                Self {

                }
            }
        }
    }
    .into()
}
