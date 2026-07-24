use std::{collections::HashMap, path::Path};

use crate::c::path;

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
    let mut acc: Vec<String> = Vec::new();
    for (i, seg) in module.iter().enumerate() {
        match seg.as_str() {
            "crate" if i == 0 => acc = vec!["crate".to_string()],
            "self" if i == 0 => acc = path::module(path)?,
            "super" => {
                if i == 0 {
                    acc = path::module(path)?;
                }
                acc.pop()?;
            }
            _ => acc.push(seg.clone()),
        }
    }
    (acc.first().map(String::as_str) == Some("crate")).then_some(acc)
}

pub fn local_impl_ok(local_defs: &[&TypeDef], path: &Path) -> bool {
    let private_same_file = local_defs.iter().any(|d| !d.is_public && d.path == path);
    if private_same_file {
        return true;
    }
    path::is_t_commons(path)
        && local_defs
            .iter()
            .any(|d| d.path == path || d.path.parent() == path.parent())
}

pub fn describe(imp: &syn::ItemImpl, self_name: &str) -> String {
    match &imp.trait_ {
        Some((trait_path, _)) => {
            let trait_name = trait_path
                .segments
                .last()
                .map(|s| s.ident.to_string())
                .unwrap_or_default();
            format!("impl {trait_name} for {self_name}")
        }
        None => format!("impl {self_name}"),
    }
}

pub fn is_ext_traits(path: &Path) -> bool {
    path::in_commons(path, "ext_traits")
}
