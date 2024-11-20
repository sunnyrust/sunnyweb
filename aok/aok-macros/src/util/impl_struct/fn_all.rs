use crate::info::DBInfo;
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn write(info: &DBInfo) -> TokenStream {
    let schema = &info.schemastruct;
    let wp = &info.aok_path;

    quote! {

        pub fn all<'args, DB>() -> #wp::query::builder::QueryBuilder<'args, Self, DB>
            where
            DB: sqlx::Database,
            #schema: #wp::table::TableColumns<DB>,
            Self: Send + Unpin + for<'r> sqlx::FromRow<'r, DB::Row>,
            {
                #wp::query::builder::QueryBuilder::new()
            }

    }
}
