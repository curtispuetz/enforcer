use std::{collections::HashSet, path::Path};

use crate::{
    c::{ast, files},
    checks::c::use_tree,
};

pub fn missing(outer: &syn::ItemUse, outer_mod: &Path) -> Vec<String> {
    let Some(c_mod) = _sub_c_mod(outer_mod) else {
        return Vec::new();
    };
    if _reexports_all(&outer.tree) {
        return Vec::new();
    }
    let referenced = _referenced_from_c(&outer.tree);
    let mut missing = Vec::new();
    for name in _inner_public_names(&c_mod) {
        if !referenced.contains(&name) {
            missing.push(name);
        }
    }
    missing
}

fn _sub_c_mod(outer_mod: &Path) -> Option<std::path::PathBuf> {
    let c_mod = outer_mod.parent()?.join("c").join("mod.rs");
    c_mod.is_file().then_some(c_mod)
}

fn _reexports_all(tree: &syn::UseTree) -> bool {
    matches!(tree, syn::UseTree::Path(p) if matches!(&*p.tree, syn::UseTree::Glob(_)))
}

fn _referenced_from_c(tree: &syn::UseTree) -> HashSet<String> {
    let mut names = HashSet::new();
    if let syn::UseTree::Path(p) = tree {
        _collect_source_names(&p.tree, &mut names);
    }
    names
}

fn _collect_source_names(tree: &syn::UseTree, names: &mut HashSet<String>) {
    match tree {
        syn::UseTree::Name(n) => {
            names.insert(n.ident.to_string());
        }
        syn::UseTree::Rename(r) => {
            names.insert(r.ident.to_string());
        }
        syn::UseTree::Path(p) => {
            names.insert(p.ident.to_string());
        }
        syn::UseTree::Group(g) => {
            for item in &g.items {
                _collect_source_names(item, names);
            }
        }
        syn::UseTree::Glob(_) => {}
    }
}

fn _inner_public_names(c_mod: &Path) -> Vec<String> {
    let mut names = Vec::new();
    for item in files::parse(c_mod).items {
        match item {
            syn::Item::Mod(m) if ast::is_public(&m.vis) => names.push(m.ident.to_string()),
            syn::Item::Use(u) if ast::is_public(&u.vis) => {
                use_tree::exposed_names(&u.tree, &mut names, &mut false);
            }
            _ => {}
        }
    }
    names
}
