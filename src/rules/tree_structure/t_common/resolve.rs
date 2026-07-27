use std::{collections::HashMap, path::Path};

use super::util;
use crate::rules::tree_structure::c::path;

use super::t::TypeDef;

pub fn target_module(
    self_name: &str,
    path: &Path,
    bindings: &HashMap<String, Vec<String>>,
) -> Option<Vec<String>> {
    match bindings.get(self_name) {
        Some(use_path) => _absolute_module(use_path, path),
        None => path::module(path),
    }
}

fn _absolute_module(use_path: &[String], path: &Path) -> Option<Vec<String>> {
    let module = use_path.get(..use_path.len().checked_sub(1)?)?;
    path::absolute(module, path)
}

pub fn local_defs<'a>(
    name: &str,
    path: &Path,
    defs: &'a HashMap<String, Vec<TypeDef>>,
    bindings: &HashMap<String, Vec<String>>,
) -> Vec<&'a TypeDef> {
    let target = target_module(name, path, bindings);
    defs.get(name)
        .into_iter()
        .flatten()
        .filter(|d| target.as_ref().is_some_and(|t| d.module.starts_with(t)))
        .collect()
}

pub fn local_impl_ok(local_defs: &[&TypeDef], path: &Path) -> bool {
    let private_same_file = local_defs.iter().any(|d| !d.is_public && d.path == path);
    if private_same_file {
        return true;
    }
    util::is_t_common(path)
        && local_defs
            .iter()
            .any(|d| d.path == path || d.path.parent() == path.parent())
}

pub fn describe(imp: &syn::ItemImpl, self_name: &str) -> String {
    match trait_base_name(imp) {
        Some(trait_name) => format!("impl {trait_name} for {self_name}"),
        None => format!("impl {self_name}"),
    }
}

pub fn trait_base_name(imp: &syn::ItemImpl) -> Option<String> {
    let (trait_path, _) = imp.trait_.as_ref()?;
    trait_path.segments.last().map(|s| s.ident.to_string())
}

pub fn is_ext_traits(path: &Path) -> bool {
    path::in_common(path, "ext_traits")
}
