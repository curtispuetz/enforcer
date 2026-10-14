use crate::rules::tree_structure::imports::t::Fix;

pub fn relative(
    resolved: &[String],
    file_dir: &[String],
    own_module: &[String],
) -> Option<Fix> {
    let rest = &resolved[file_dir.len()..];
    if rest.is_empty() {
        return None;
    }
    // not-obvious: a mod.rs/lib.rs is its folder's module, so `super` would leave
    // the folder; its children are reached by a bare path instead.
    let path = rest.join("::");
    if own_module.len() == file_dir.len() {
        Some(Fix::Rewrite(format!("use {path};")))
    } else {
        Some(Fix::Rewrite(format!("use super::{path};")))
    }
}

pub fn absolute(resolved: &[String]) -> Fix {
    Fix::Rewrite(format!("use {};", resolved.join("::")))
}
