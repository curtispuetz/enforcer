use std::collections::HashSet;

use crate::s::EXISTING_SRC_DIRS;

use super::files;

pub fn defined() -> HashSet<String> {
    let mut ret = HashSet::new();
    for dir_name in EXISTING_SRC_DIRS.iter() {
        for file_path in files::rs(dir_name) {
            for item in files::parse(&file_path).items {
                if let Some(name) = _type_name(&item) {
                    ret.insert(name);
                }
            }
        }
    }
    ret
}

fn _type_name(item: &syn::Item) -> Option<String> {
    match item {
        syn::Item::Struct(i) => Some(i.ident.to_string()),
        syn::Item::Enum(i) => Some(i.ident.to_string()),
        syn::Item::Union(i) => Some(i.ident.to_string()),
        syn::Item::Type(i) => Some(i.ident.to_string()),
        _ => None,
    }
}
