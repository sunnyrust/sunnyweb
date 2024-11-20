use crate::info::DBInfo;
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn write(info: &DBInfo) -> TokenStream {
    let wp = &info.aok_path;
    quote! {

    pub async fn from_raw_sql<'a, 'c, DB, C>(
        sql: &'static str,
        arguments: <DB as sqlx::database::HasArguments<'a>>::Arguments,
        conn: &'c C,
    ) -> #wp::errors::Result<Vec<#wp::state::DbState<Self>>>
    where
        'c: 'a,
        DB: sqlx::Database,
        C: #wp::connection::Connection<DB>,
        Self: Send + Unpin + for<'r> sqlx::FromRow<'r, DB::Row>,
    {
        let mut data: Vec<Self> = conn.fetch_all(sql, arguments).await?;

        Ok(data
            .drain(..)
            .map(|x| #wp::state::DbState::db_loaded(x))
            .collect())
    }

    }
}
