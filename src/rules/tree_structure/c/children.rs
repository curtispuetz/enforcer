use std::path::{Path, PathBuf};

use syn::{File, Item};

use crate::rules::c::{ast, path};

pub fn public(file: &Path, ast: &File) -> Vec<(String, PathBuf)> {
    let mut ret = Vec::new();
    for item in &ast.items {
        if let Item::Mod(m) = item
            && m.content.is_none()
            && ast::is_public(&m.vis)
            && let Some(child) = find(file, &m.ident.to_string())
        {
            ret.push((m.ident.to_string(), child));
        }
    }
    ret
}

pub fn find(file: &Path, name: &str) -> Option<PathBuf> {
    if !path::is_mod_or_lib(file) {
        return None;
    }
    let dir = file.parent()?;
    let flat = dir.join(format!("{name}.rs"));
    if flat.is_file() {
        return Some(flat);
    }
    let nested = dir.join(name).join("mod.rs");
    nested.is_file().then_some(nested)
}
