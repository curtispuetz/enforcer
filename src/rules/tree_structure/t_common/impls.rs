use std::{collections::HashMap, path::Path};

use crate::rules::c::imports;

use super::{resolve, t::TypeDef};

pub fn misplaced(
    file: &syn::File,
    path: &Path,
    type_defs: &HashMap<String, Vec<TypeDef>>,
) -> Vec<String> {
    let bindings = imports::bindings(file);
    let mut items = Vec::new();
    for item in &file.items {
        if let syn::Item::Impl(imp) = item
            && let Some(desc) = _misplaced_impl(imp, path, type_defs, &bindings)
        {
            items.push(desc);
        }
    }
    items
}

fn _misplaced_impl(
    imp: &syn::ItemImpl,
    path: &Path,
    type_defs: &HashMap<String, Vec<TypeDef>>,
    bindings: &HashMap<String, Vec<String>>,
) -> Option<String> {
    let self_name = _self_base_name(&imp.self_ty)?;
    let is_trait = imp.trait_.is_some();
    let target = resolve::target_module(&self_name, path, bindings);
    let local_defs: Vec<&TypeDef> = type_defs
        .get(&self_name)
        .into_iter()
        .flatten()
        .filter(|d| target.as_ref().is_some_and(|t| d.module.starts_with(t)))
        .collect();
    let valid = if local_defs.is_empty() {
        !is_trait || resolve::is_ext_traits(path)
    } else {
        resolve::local_impl_ok(&local_defs, path)
    };
    if valid {
        None
    } else {
        Some(resolve::describe(imp, &self_name))
    }
}

pub fn _self_base_name(ty: &syn::Type) -> Option<String> {
    match ty {
        syn::Type::Path(p) => p.path.segments.last().map(|s| s.ident.to_string()),
        syn::Type::Reference(r) => _self_base_name(&r.elem),
        _ => None,
    }
}
