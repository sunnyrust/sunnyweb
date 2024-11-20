use crate::info::DBInfo;
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn write(info: &DBInfo) -> TokenStream {
    let wp = &info.aok_path;
    let cols: Vec<_> = info
        .columns
        .iter()
        .map(|c| {
            let name = &c.field;
            quote! { #name: Default::default() }
        })
        .collect();
    let cols = quote! { #(#cols),* };

    quote! {

        pub fn new() -> #wp::state::DbState<Self> {
            #wp::state::DbState::new_uncreated(Self {
                #cols
            })
        }

    }
}
