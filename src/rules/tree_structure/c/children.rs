use {
    crate::rules::c::{ast, path},
    std::path::{Path, PathBuf},
    syn::{File, Item},
};

pub fn public(file: &Path, ast: &File) -> Vec<(String, PathBuf)> {
    let mut ret = Vec::new();
    for item in &ast.items {
        if let Item::Mod(m) = item
            && m.content.is_none()
            && ast::is_public(&m.vis)
            && let Some(child) = path::child(file, &m.ident.to_string())
        {
            ret.push((m.ident.to_string(), child));
        }
    }
    ret
}
