use std::{collections::HashSet, path::Path};

use crate::c::{ast, files};

pub fn names(mod_rs: &Path) -> (bool, HashSet<String>) {
    let mut glob = false;
    let mut names = HashSet::new();
    for item in files::parse(mod_rs).items {
        if let syn::Item::Use(u) = &item
            && ast::is_public(&u.vis)
        {
            _collect(&u.tree, &mut names, &mut glob);
        }
    }
    (glob, names)
}

fn _collect(tree: &syn::UseTree, names: &mut HashSet<String>, glob: &mut bool) {
    match tree {
        syn::UseTree::Name(n) => {
            names.insert(n.ident.to_string());
        }
        syn::UseTree::Rename(r) => {
            names.insert(r.rename.to_string());
        }
        syn::UseTree::Path(p) => _collect(&p.tree, names, glob),
        syn::UseTree::Group(g) => {
            for item in &g.items {
                _collect(item, names, glob);
            }
        }
        syn::UseTree::Glob(_) => *glob = true,
    }
}
