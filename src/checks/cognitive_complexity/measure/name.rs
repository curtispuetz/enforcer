use syn::{Ident, Type};

pub fn of(ident: &Ident, type_name: Option<&str>) -> String {
    match type_name {
        Some(type_name) => format!("{type_name}::{ident}"),
        None => ident.to_string(),
    }
}

pub fn self_type(ty: &Type) -> Option<String> {
    let Type::Path(path) = ty else {
        return None;
    };
    Some(path.path.segments.last()?.ident.to_string())
}
