use std::{collections::HashMap, path::Path};

use crate::{
    c::{files, path, scan},
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
    let mut items = Vec::new();
    for item in files::parse(path).items {
        if let syn::Item::Impl(imp) = &item
            && let Some(desc) = _misplaced_impl(imp, path, type_defs)
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
) -> Option<String> {
    let self_name = _self_base_name(&imp.self_ty)?;
    let is_trait = imp.trait_.is_some();
    let valid = match type_defs.get(&self_name) {
        None => !is_trait || _is_ext_traits(path),
        Some(local_defs) => _local_impl_ok(local_defs, path),
    };
    if valid {
        None
    } else {
        Some(_describe(imp, &self_name))
    }
}

fn _local_impl_ok(local_defs: &[TypeDef], path: &Path) -> bool {
    let private_same_file = local_defs.iter().any(|d| !d.is_public && d.path == path);
    if private_same_file {
        return true;
    }
    _is_t_commons(path)
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

fn _is_t_commons(path: &Path) -> bool {
    path::under_dir(path, "t") || path::commons_file_kind(path) == Some("t")
}
