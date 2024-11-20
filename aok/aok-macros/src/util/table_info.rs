use crate::info::DBInfo;
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn write(info: &DBInfo) -> TokenStream {
    let wp = &info.aok_path;
    let mut parts = Vec::default();
    let tn = &info.tablename;
    parts.push(quote! { #tn });

    if let Some(namespace) = &info.schemaname {
        parts.push(quote! { #namespace });
    }

    let parts: Vec<_> = parts.drain(..).rev().collect();
    let schema = &info.schemastruct;

    quote! {

        impl #wp::table::TableInfo for #schema {
            fn identifier() -> &'static [&'static str] {
                &[#(#parts),*]
            }
        }

    }
}
