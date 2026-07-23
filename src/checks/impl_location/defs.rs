use std::collections::HashMap;

use crate::{
    c::{ast, files},
    s::EXISTING_SRC_DIRS,
};

use super::t::TypeDef;

// not-obvious: We index every type definition in the crate by name so the check can
// tell whether an impl's self type is local (and where it is defined) or foreign.
pub fn type_defs() -> HashMap<String, Vec<TypeDef>> {
    let mut map: HashMap<String, Vec<TypeDef>> = HashMap::new();
    for dir_name in EXISTING_SRC_DIRS.iter() {
        for path in files::rs(dir_name) {
            for item in files::parse(&path).items {
                if let Some((name, is_public)) = _type_def(&item) {
                    map.entry(name).or_default().push(TypeDef {
                        path: path.clone(),
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
