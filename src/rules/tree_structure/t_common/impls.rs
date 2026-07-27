use std::{collections::HashMap, path::Path};

use crate::rules::c::imports;

use super::{resolve, t::Defs};

pub fn misplaced(file: &syn::File, path: &Path, defs: &Defs) -> Vec<String> {
    let imported = imports::bindings(file);
    let mut items = Vec::new();
    for item in &file.items {
        if let syn::Item::Impl(imp) = item
            && let Some(desc) = _misplaced_impl(imp, path, defs, &imported)
        {
            items.push(desc);
        }
    }
    items
}

fn _misplaced_impl(
    imp: &syn::ItemImpl,
    path: &Path,
    defs: &Defs,
    bindings: &HashMap<String, Vec<String>>,
) -> Option<String> {
    let self_name = _self_base_name(&imp.self_ty)?;
    let trait_name = resolve::trait_base_name(imp);
    if _is_foreign_trait(trait_name.as_deref(), path, defs, bindings) {
        return None;
    }
    let local_defs = resolve::local_defs(&self_name, path, &defs.types, bindings);
    let valid = if local_defs.is_empty() {
        trait_name.is_none() || resolve::is_ext_traits(path)
    } else {
        resolve::local_impl_ok(&local_defs, path)
    };
    if valid {
        None
    } else {
        Some(resolve::describe(imp, &self_name))
    }
}

fn _is_foreign_trait(
    trait_name: Option<&str>,
    path: &Path,
    defs: &Defs,
    bindings: &HashMap<String, Vec<String>>,
) -> bool {
    match trait_name {
        Some(name) => {
            resolve::local_defs(name, path, &defs.traits, bindings).is_empty()
        }
        None => false,
    }
}

pub fn _self_base_name(ty: &syn::Type) -> Option<String> {
    match ty {
        syn::Type::Path(p) => p.path.segments.last().map(|s| s.ident.to_string()),
        syn::Type::Reference(r) => _self_base_name(&r.elem),
        _ => None,
    }
}
