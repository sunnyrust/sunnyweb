use syn::Ident;

use crate::{attributes,
    relation::Relation,
    column::Column,
    types::Result
};

pub(crate) struct DBInfo {
    pub dbstruct: Ident,
    pub schemastruct: Ident,
    pub tablename: String,
    pub columns: Vec<Column>,
    pub pks: Vec<Column>,
    pub schemaname: Option<String>,
    pub aok_path: syn::Path,
    pub readonly: bool,
    pub engines_ident: Vec<Ident>,
    pub relations: Vec<Relation>,
    pub relations_struct: Ident,
}

impl DBInfo {
    pub fn new(ast: &syn::DeriveInput) -> Result<Self> {
        let dbstruct = attributes::get_structname(ast);
        let schemastruct_name = format!("{}Schema", dbstruct);
        let schemastruct = Ident::new(&schemastruct_name, dbstruct.span());
        let relations_struct_name = format!("{}Relation", dbstruct);
        let relations_struct = Ident::new(&relations_struct_name, dbstruct.span());
        let aok_path = attributes::get_aok_path(ast);
        let tablename = attributes::get_tablename(ast);
        let schemaname = attributes::get_schemaname(ast);
        let columns = attributes::get_columns(ast);
        let pks = attributes::get_pks(ast);
        let readonly = attributes::get_readonly(ast);
        let engines = attributes::get_engines(ast);
        let relations = attributes::get_relations(ast)?;
        let engines_ident = engines
            .iter()
            .map(|e| Ident::new(e.as_str(), dbstruct.span()))
            .collect();
        Ok(Self { 
            dbstruct ,
            schemastruct,
            tablename,
            schemaname,
            aok_path,
            columns,
            pks,
            readonly,
            engines_ident,
            relations,
            relations_struct,
        })
    }
}