use crate::info::DBInfo;
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn write(info: &DBInfo) -> TokenStream {
    let defstruct = &info.dbstruct;
    let wp = &info.aok_path;
    let schemastruct = &info.schemastruct;

    quote! {
        impl #wp::table::HasSchema for #defstruct {
            type Schema = #schemastruct;
        }
    }
}
