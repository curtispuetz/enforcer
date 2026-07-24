use std::{collections::HashSet, path::Path};

use crate::c::{ast, files, path};

pub fn violations(path: &Path, crate_types: &HashSet<String>) -> Vec<String> {
    let mut ret = Vec::new();
    for item in files::ast_parse(path).items {
        match item {
            syn::Item::Trait(t) if !path::in_commons(path, "ext_traits") => {
                ret.push(format!("trait {} defined outside ext_traits", t.ident));
            }
            syn::Item::Impl(imp) => {
                if let Some(desc) = _crate_type_impl(&imp, crate_types) {
                    ret.push(desc);
                }
            }
            _ => {}
        }
    }
    ret
}

fn _crate_type_impl(
    imp: &syn::ItemImpl,
    crate_types: &HashSet<String>,
) -> Option<String> {
    let (trait_path, _) = imp.trait_.as_ref()?;
    let self_name = ast::self_base_name(&imp.self_ty)?;
    if !crate_types.contains(&self_name) {
        return None;
    }
    let trait_name = trait_path
        .segments
        .last()
        .map(|s| s.ident.to_string())
        .unwrap_or_default();
    Some(format!("impl {trait_name} for {self_name}"))
}
