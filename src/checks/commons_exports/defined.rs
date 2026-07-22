use std::path::{Path, PathBuf};

use crate::c::{ast, files};

use super::t::Kind;

pub fn public_names(mod_rs: &Path, kind: Kind) -> Vec<String> {
    let mut names = Vec::new();
    for file in _dir_files(mod_rs) {
        for item in files::parse(&file).items {
            if let Some(name) = _public_name(&item, kind) {
                names.push(name);
            }
        }
    }
    names
}

fn _dir_files(mod_rs: &Path) -> Vec<PathBuf> {
    let Some(dir) = mod_rs.parent() else {
        return Vec::new();
    };
    let pattern = format!("{}/**/*.rs", dir.to_string_lossy().replace('\\', "/"));
    glob::glob(&pattern)
        .expect("invalid glob")
        .filter_map(|p| p.ok())
        .collect()
}

fn _public_name(item: &syn::Item, kind: Kind) -> Option<String> {
    match kind {
        Kind::Types => _public_type(item),
        Kind::Statics => _public_static(item),
    }
}

fn _public_type(item: &syn::Item) -> Option<String> {
    match item {
        syn::Item::Struct(i) if ast::is_public(&i.vis) => Some(i.ident.to_string()),
        syn::Item::Enum(i) if ast::is_public(&i.vis) => Some(i.ident.to_string()),
        syn::Item::Trait(i) if ast::is_public(&i.vis) => Some(i.ident.to_string()),
        syn::Item::Type(i) if ast::is_public(&i.vis) => Some(i.ident.to_string()),
        _ => None,
    }
}

fn _public_static(item: &syn::Item) -> Option<String> {
    match item {
        syn::Item::Static(i) if ast::is_public(&i.vis) => Some(i.ident.to_string()),
        _ => None,
    }
}
