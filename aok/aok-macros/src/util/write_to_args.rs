use crate::column::Column;
use crate::info::DBInfo;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

pub(crate) fn write(info: &DBInfo) -> TokenStream {
    // If this is a readonly model it should NOT impl WriteToArgs
    if info.readonly {
        return quote!();
    }

    let mut blocks = Vec::default();

    for db in &info.engines_ident {
        let write_col = if db.to_string().as_str() == "Sqlite" {
            write_col_sqlite
        } else {
            write_col_normal
        };

        let fields: Vec<_> = info
            .columns
            .iter()
            .filter(|x| !x.ignore)
            .map(write_col)
            .collect();
        let fields = quote! { #(#fields)* };

        blocks.push(write_for_db(info, db, &fields));
    }

    quote! { #(#blocks)* }
}

pub(crate) fn write_col_normal(col: &Column) -> TokenStream {
    let dbname = col.dbname.as_str();
    let field = &col.field;
    quote! { #dbname => args.add(&self.#field), }
}

pub(crate) fn write_col_sqlite(col: &Column) -> TokenStream {
    let dbname = col.dbname.as_str();
    let field = &col.field;
    quote! { #dbname => args.add(self.#field.clone()), }
}

pub(crate) fn write_for_db(info: &DBInfo, db: &Ident, matches: &TokenStream) -> TokenStream {
    let def = &info.dbstruct;
    let wp = &info.aok_path;

    quote! {

    impl #wp::table::WriteToArgs<sqlx::#db> for #def {
        fn bind<'args>(
            &self,
            column: &str,
            args: &mut <sqlx::#db as sqlx::database::HasArguments<'args>>::Arguments,
        ) -> #wp::errors::Result<()> {
            use sqlx::Arguments;
            match column {
                #matches
                _ => {
                    return Err(#wp::errors::AokError::MissingDbColumn(
                        column.to_owned(),
                    ).into())
                }
            }
            Ok(())
        }
    }

        }
}
