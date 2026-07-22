use std::path::Path;

use crate::c::{ast, files};

use super::exported;

pub fn of(mod_rs: &Path) -> Vec<String> {
    let globs = exported::glob_modules(mod_rs);
    let mut issues = Vec::new();
    for item in files::parse(mod_rs).items {
        if let syn::Item::Mod(m) = &item {
            let name = m.ident.to_string();
            if ast::is_public(&m.vis) {
                issues.push(format!("module `{name}` is not private"));
            }
            if !globs.contains(&name) {
                issues.push(format!(
                    "module `{name}` is missing its glob re-export (`pub use {name}::*;`)"
                ));
            }
        }
    }
    issues
}
