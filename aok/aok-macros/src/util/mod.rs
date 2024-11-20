mod define_schema;
mod has_schema;
mod impl_struct;
mod relations;
mod table_columns;
mod table_info;
mod unique_identifier;
mod write_bulk_array_to_args;
mod write_to_args;

pub(crate) use define_schema::write as define_schema;
pub(crate) use has_schema::write as has_schema;
pub(crate) use impl_struct::write as impl_struct;
pub(crate) use relations::write as relations;
pub(crate) use table_columns::write as table_columns;
pub(crate) use table_info::write as table_info;
pub(crate) use unique_identifier::write as unique_identifier;
pub(crate) use write_bulk_array_to_args::write as write_bulk_array_to_args;
pub(crate) use write_to_args::write as write_to_args;



use syn::{Type, TypeGroup, TypePath};

pub(crate) fn get_clause(ty: &Type, nullable: bool) -> String {
    let base_type = match ty {
        syn::Type::Path(tp) => get_clause_typepath(tp),
        syn::Type::Group(g) => get_clause_typegroup(g),
        _ => None,
    }
    .unwrap_or("Basic")
    .to_owned();
    if nullable {
        return format!("{}Opt", base_type);
    }
    base_type
}

fn get_clause_typegroup(_ty: &TypeGroup) -> Option<&'static str> {
    // If we ever need a special clause for Vec<T>
    None
}

fn get_clause_typepath(ty: &TypePath) -> Option<&'static str> {
    let ident = ty
        .path
        .get_ident()
        .or(ty.path.segments.first().map(|f| &f.ident))?;
    let name = ident.to_string();
    let clause = match name.as_str() {
        "u8" => "Numeric",
        "i8" => "Numeric",
        "u16" => "Numeric",
        "i16" => "Numeric",
        "u32" => "Numeric",
        "i32" => "Numeric",
        "u64" => "Numeric",
        "i64" => "Numeric",
        "f32" => "Numeric",
        "f64" => "Numeric",
        "String" => "Text",
        "chrono" => "Numeric",
        "PgMoney" => "Numeric",
        _ => return None,
    };
    Some(clause)
}

pub(crate) fn as_typepath(ty: &syn::Type) -> Option<&syn::TypePath> {
    match ty {
        syn::Type::Path(tp) => Some(tp),
        _ => None,
    }
}
