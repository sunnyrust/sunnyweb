use crate::{info::DBInfo, relation::Relation};
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn write(info: &DBInfo) -> TokenStream {
    let wp = &info.aok_path;
    let defstruct = &info.dbstruct;
    let relations_struct = &info.relations_struct;
    let relations = info.relations.as_slice();
    if relations.is_empty() {
        return quote! {};
    }

    let struct_fields: Vec<_> = relations.iter().map(|x| fielddef(info, x)).collect();
    let struct_fields = quote! { #(#struct_fields), * };
    let default_fields: Vec<_> = relations.iter().map(|x| defaultdef(info, x)).collect();
    let default_fields = quote! { #(#default_fields), * };

    quote! {

        impl #wp::relations::HasRelations for #defstruct {
            type Relation = #relations_struct;
        }

        pub struct #relations_struct {
            #struct_fields
        }

        impl Default for #relations_struct {
            fn default() -> Self {
                Self {
                    #default_fields
                }
            }
        }

    }
}

fn fielddef(info: &DBInfo, relation: &Relation) -> TokenStream {
    let wp = &info.aok_path;
    let kind = &relation.kind;
    let field = &relation.field;
    let other = &relation.foreign_struct;
    quote! {
        pub #field: #wp::relations::#kind<#other>
    }
}

fn defaultdef(info: &DBInfo, relation: &Relation) -> TokenStream {
    let wp = &info.aok_path;
    let kind = &relation.kind;
    let field = &relation.field;
    let fk = &relation.foreign_key;
    quote! {
        #field: #wp::relations::#kind::using(#fk)
    }
}
