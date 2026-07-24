use std::collections::HashMap;

use crate::{
    c::{ast, files, path},
    s::EXISTING_SRC_DIRS,
};

use super::t::TypeDef;

pub fn find() -> HashMap<String, Vec<TypeDef>> {
    let mut map: HashMap<String, Vec<TypeDef>> = HashMap::new();
    for dir_name in EXISTING_SRC_DIRS.iter() {
        for file_path in files::rs(dir_name) {
            let module = path::module(&file_path).unwrap_or_default();
            for item in files::ast_parse(&file_path).items {
                if let Some((name, is_public)) = _type_def(&item) {
                    map.entry(name).or_default().push(TypeDef {
                        path: file_path.clone(),
                        module: module.clone(),
                        is_public,
                    });
                }
            }
        }
    }
    map
}

fn _type_def(item: &syn::Item) -> Option<(String, bool)> {
    match item {
        syn::Item::Struct(i) => Some((i.ident.to_string(), ast::is_public(&i.vis))),
        syn::Item::Enum(i) => Some((i.ident.to_string(), ast::is_public(&i.vis))),
        syn::Item::Union(i) => Some((i.ident.to_string(), ast::is_public(&i.vis))),
        syn::Item::Type(i) => Some((i.ident.to_string(), ast::is_public(&i.vis))),
        _ => None,
    }
}
