use proc_macro::TokenStream;
use quote::quote;
pub(crate) mod types;
pub(crate) mod attributes;
pub(crate) mod info;
pub(crate) mod util;
pub(crate) mod column;
pub(crate) mod engine;
pub(crate) mod relation;


use info::DBInfo;
// use types::*;

/// This is the main macro that is called by the user
#[proc_macro_derive(AokModel, attributes(aok, aok_path))]
pub fn model_gen(input: TokenStream) -> TokenStream {
    match model_gen_inner(input) {
        Ok(q) => q,
        Err(err) => quote! { std::compile_error!(#err); }.into(),
    }
}


fn model_gen_inner(input: TokenStream) -> types::Result<TokenStream> {
    // Gather the Info needed to build all the code snipits
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let info = DBInfo::new(&ast)?;
    let p1 = util::has_schema(&info);
    let p2 = util::write_to_args(&info);
    let p3 = util::write_bulk_array_to_args(&info);
    let p4 = util::define_schema(&info);
    let p5 = util::table_info(&info);
    let p6 = util::table_columns(&info);
    let p7  = util::relations(&info);
    let p8 = util::unique_identifier(&info);
    let p9 = util::impl_struct(&info);
    // Build the code snipits
    let q = quote! {
        #p1
        #p2
        #p3
        #p4
        #p5
        #p6
        #p7
        #p8
        #p9
    };

    // Want to see what the macros generate?
    let code = q.to_string();
    let _=std::fs::create_dir_all("/tmp/aokmacro/");
    let filename = format!(
        "/tmp/aokmacro/{}.rs",
        info.dbstruct.to_string().to_lowercase()
    );
    let _=std::fs::write(filename, code);
    // Return the generated code
    Ok(q.into())
}
#[cfg(test)]
mod tests {
    use super::*;

}
