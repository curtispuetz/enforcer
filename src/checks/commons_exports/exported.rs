use std::{collections::HashSet, path::Path};

use crate::{
    c::{ast, files},
    checks::c::use_tree,
};

pub fn names(mod_rs: &Path) -> (bool, HashSet<String>) {
    let mut glob = false;
    let mut names = Vec::new();
    for item in files::parse(mod_rs).items {
        if let syn::Item::Use(u) = &item
            && ast::is_public(&u.vis)
        {
            use_tree::exposed_names(&u.tree, &mut names, &mut glob);
        }
    }
    (glob, names.into_iter().collect())
}
