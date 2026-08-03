use {
    super::t::{Defs, TypeDef},
    crate::{
        rules::{
            c::{ast, files},
            tree_structure::c::path,
        },
        s::EXISTING_SRC_DIRS,
    },
    std::{collections::HashMap, path::Path},
};

pub fn find() -> Defs {
    let mut defs = Defs {
        types: HashMap::new(),
        traits: HashMap::new(),
    };
    for dir_name in EXISTING_SRC_DIRS.iter() {
        for file_path in files::rs(dir_name) {
            _add_file(&file_path, &mut defs);
        }
    }
    defs
}

fn _add_file(file_path: &Path, defs: &mut Defs) {
    let module = path::module(file_path).unwrap_or_default();
    for item in files::ast_parse(file_path).items {
        if let Some((name, is_public, is_trait)) = _def(&item) {
            let map = if is_trait {
                &mut defs.traits
            } else {
                &mut defs.types
            };
            map.entry(name).or_default().push(TypeDef {
                path: file_path.to_path_buf(),
                module: module.clone(),
                is_public,
            });
        }
    }
}

fn _def(item: &syn::Item) -> Option<(String, bool, bool)> {
    match item {
        syn::Item::Struct(i) => {
            Some((i.ident.to_string(), ast::is_public(&i.vis), false))
        }
        syn::Item::Enum(i) => {
            Some((i.ident.to_string(), ast::is_public(&i.vis), false))
        }
        syn::Item::Union(i) => {
            Some((i.ident.to_string(), ast::is_public(&i.vis), false))
        }
        syn::Item::Type(i) => {
            Some((i.ident.to_string(), ast::is_public(&i.vis), false))
        }
        syn::Item::Trait(i) => {
            Some((i.ident.to_string(), ast::is_public(&i.vis), true))
        }
        _ => None,
    }
}
