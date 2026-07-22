use std::{collections::HashSet, path::Path};

use crate::c::{ast, files};

pub fn glob_modules(mod_rs: &Path) -> HashSet<String> {
    let mut modules = HashSet::new();
    for item in files::parse(mod_rs).items {
        if let syn::Item::Use(u) = &item
            && ast::is_public(&u.vis)
            && let Some(module) = ast::glob_module(&u.tree)
        {
            modules.insert(module);
        }
    }
    modules
}
