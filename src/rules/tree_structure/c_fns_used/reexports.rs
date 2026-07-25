use std::{collections::HashSet, path::Path};

use crate::{
    rules::{
        c::{ast, files, path},
        tree_structure::c::{ast as ast2, path as path2},
    },
    s::EXISTING_SRC_DIRS,
};

pub fn aliases() -> HashSet<Vec<String>> {
    let mut raw = _raw();
    raw.sort_by_key(Vec::len);
    let mut set: HashSet<Vec<String>> = HashSet::new();
    for module in raw {
        set.insert(canonical(&module, &set));
    }
    set
}

pub fn canonical(module: &[String], aliases: &HashSet<Vec<String>>) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for seg in module {
        if seg == "c" && aliases.contains(&out) {
            continue;
        }
        out.push(seg.clone());
    }
    out
}

pub fn module_of(file: &Path, aliases: &HashSet<Vec<String>>) -> Option<Vec<String>> {
    Some(canonical(&path2::module(file)?, aliases))
}

fn _raw() -> Vec<Vec<String>> {
    let mut ret = Vec::new();
    for dir_name in EXISTING_SRC_DIRS.iter() {
        for file in files::rs(dir_name) {
            if !_reexports_c(&file) {
                continue;
            }
            let Some(module) = path2::module(&file) else {
                continue;
            };
            ret.push(module);
        }
    }
    ret
}

fn _reexports_c(file: &Path) -> bool {
    if !path::is_mod_or_lib(file) || !path2::under_dir(file, "c") {
        return false;
    }
    files::ast_parse(file).items.iter().any(_is_c_glob)
}

fn _is_c_glob(item: &syn::Item) -> bool {
    let syn::Item::Use(u) = item else {
        return false;
    };
    ast::is_public(&u.vis) && ast2::glob_module(&u.tree) == Some("c".to_string())
}
