use std::collections::{HashMap, HashSet};

use super::message;

pub fn check(
    file: &syn::File,
    rel: &str,
    is_ext: bool,
    public_defs: &HashMap<String, Vec<String>>,
    crate_types: &HashSet<String>,
) -> Vec<String> {
    let mut out = Vec::new();
    for item in &file.items {
        let syn::Item::Impl(imp) = item else {
            continue;
        };
        let Some(name) = _self_ident(imp) else {
            continue;
        };
        if imp.trait_.is_some() && !is_ext && !crate_types.contains(&name) {
            out.push(message::foreign_trait_impl(&name));
        } else if !is_ext && _misplaced(&name, rel, public_defs) {
            out.push(message::misplaced_impl(&name));
        }
    }
    out
}

fn _misplaced(
    name: &str,
    rel: &str,
    public_defs: &HashMap<String, Vec<String>>,
) -> bool {
    public_defs
        .get(name)
        .is_some_and(|files| !files.iter().any(|f| f == rel))
}

fn _self_ident(imp: &syn::ItemImpl) -> Option<String> {
    let syn::Type::Path(p) = &*imp.self_ty else {
        return None;
    };
    p.path.segments.last().map(|s| s.ident.to_string())
}
