use std::{collections::HashSet, path::Path};

use crate::c::{ast, files};

pub fn names(mod_rs: &Path) -> (bool, HashSet<String>) {
    let mut glob = false;
    let mut names = Vec::new();
    for item in files::parse(mod_rs).items {
        if let syn::Item::Use(u) = &item
            && ast::is_public(&u.vis)
        {
            ast::exposed_names(&u.tree, &mut names, &mut glob);
        }
    }
    (glob, names.into_iter().collect())
}
