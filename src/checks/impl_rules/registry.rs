use std::collections::{HashMap, HashSet};

use crate::{
    c::{ast, files, path},
    s::EXISTING_SRC_DIRS,
};

pub fn build() -> (HashMap<String, Vec<String>>, HashSet<String>) {
    let mut public_defs: HashMap<String, Vec<String>> = HashMap::new();
    let mut all = HashSet::new();
    for dir_name in EXISTING_SRC_DIRS.iter() {
        for path in files::rs(dir_name) {
            let rel = path::rel(&path);
            for item in &files::parse(&path).items {
                if let Some((name, public)) = _type_def(item) {
                    if public {
                        public_defs.entry(name.clone()).or_default().push(rel.clone());
                    }
                    all.insert(name);
                }
            }
        }
    }
    (public_defs, all)
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
