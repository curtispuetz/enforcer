use std::{collections::HashMap, path::Path};

use crate::{
    c::{ast, files, imports, path, scan},
    t::{ItemsViolation, Outcome},
};

use super::{report, resolve, t::TypeDef, type_defs};

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
    let self_name = ast::self_base_name(&imp.self_ty)?;
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
