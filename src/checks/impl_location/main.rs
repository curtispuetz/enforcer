use std::{collections::HashMap, path::Path};

use crate::{
    c::{files, imports, path, scan},
    t::{ItemsViolation, Outcome},
};

use super::{report, t::TypeDef, type_defs};

pub fn run() -> bool {
    let type_defs = type_defs::find();
    let (passed, violations) = scan::src_files(|path| _check_file(path, &type_defs));
    report::print(passed, violations)
}

fn _check_file(
    path: &Path,
    type_defs: &HashMap<String, Vec<TypeDef>>,
) -> Outcome<ItemsViolation> {
    let file = files::parse(path);
    let bindings = imports::bindings(&file);
    let mut items = Vec::new();
    for item in &file.items {
        if let syn::Item::Impl(imp) = item
            && let Some(desc) = _misplaced_impl(imp, path, type_defs, &bindings)
        {
            items.push(desc);
        }
    }
    if items.is_empty() {
        Outcome::Passed
    } else {
        Outcome::Failed(ItemsViolation {
            path: path::rel(path),
            items,
        })
    }
}

fn _misplaced_impl(
    imp: &syn::ItemImpl,
    path: &Path,
    type_defs: &HashMap<String, Vec<TypeDef>>,
    bindings: &HashMap<String, Vec<String>>,
) -> Option<String> {
    let self_name = _self_base_name(&imp.self_ty)?;
    let is_trait = imp.trait_.is_some();
    let target = _target_module(&self_name, path, bindings);
    let local_defs: Vec<&TypeDef> = type_defs
        .get(&self_name)
        .into_iter()
        .flatten()
        .filter(|d| target.as_ref().is_some_and(|t| d.module.starts_with(t)))
        .collect();
    let valid = if local_defs.is_empty() {
        !is_trait || _is_ext_traits(path)
    } else {
        _local_impl_ok(&local_defs, path)
    };
    if valid {
        None
    } else {
        Some(_describe(imp, &self_name))
    }
}

fn _target_module(
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

fn _local_impl_ok(local_defs: &[&TypeDef], path: &Path) -> bool {
    let private_same_file = local_defs.iter().any(|d| !d.is_public && d.path == path);
    if private_same_file {
        return true;
    }
    path::is_t_commons(path)
        && local_defs
            .iter()
            .any(|d| d.path == path || d.path.parent() == path.parent())
}

fn _self_base_name(ty: &syn::Type) -> Option<String> {
    match ty {
        syn::Type::Path(p) => p.path.segments.last().map(|s| s.ident.to_string()),
        syn::Type::Reference(r) => _self_base_name(&r.elem),
        _ => None,
    }
}

fn _describe(imp: &syn::ItemImpl, self_name: &str) -> String {
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

fn _is_ext_traits(path: &Path) -> bool {
    path::under_dir(path, "ext_traits")
        || path::commons_file_kind(path) == Some("ext_traits")
}
