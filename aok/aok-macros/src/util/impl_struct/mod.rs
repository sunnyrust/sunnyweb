use crate::info::DBInfo;
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) mod fn_all;
pub(crate) mod fn_find_by_id;
pub(crate) mod fn_from_raw_sql;
pub(crate) mod fn_new;
pub(crate) mod fn_select;
pub(crate) mod fn_where_col;

pub(crate) fn write(infos: &DBInfo) -> TokenStream {
    let defstruct = &infos.dbstruct;

    let p1 = fn_new::write(infos);
    let p2 = fn_all::write(infos);
    let p3 = fn_where_col::write(infos);
    let p4 = fn_find_by_id::write(infos);
    let p5 = fn_from_raw_sql::write(infos);
    let p6 = fn_select::write(infos);

    quote! {

        impl #defstruct {
            #p1
            #p2
            #p3
            #p4
            #p5
            #p6
        }

    }
}
