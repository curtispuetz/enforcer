use std::{collections::HashSet, path::Path};

use crate::rules::c::{ast, files};

use super::exported;

pub fn of(mod_rs: &Path) -> Vec<String> {
    let globs = exported::glob_modules(mod_rs);
    let mut issues = Vec::new();
    for item in files::ast_parse(mod_rs).items {
        if let syn::Item::Mod(m) = &item {
            issues.extend(_mod_issues(mod_rs, m, &globs));
        }
    }
    issues
}

fn _mod_issues(mod_rs: &Path, m: &syn::ItemMod, globs: &HashSet<String>) -> Vec<String> {
    let name = m.ident.to_string();
    let mut issues = Vec::new();
    if ast::is_public(&m.vis) {
        issues.push(format!("module `{name}` is not private"));
    }
    if !globs.contains(&name) && exported::has_public_items(mod_rs, &name) {
        issues.push(format!(
            "module `{name}` is missing its glob re-export (`pub use {name}::*;`)"
        ));
    }
    issues
}
