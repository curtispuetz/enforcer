use syn::UseTree;

use crate::rules::c::ast;

pub fn trees(file: &syn::File) -> Vec<&UseTree> {
    file.items
        .iter()
        .filter_map(|item| match item {
            syn::Item::Use(u) if ast::is_public(&u.vis) => Some(&u.tree),
            _ => None,
        })
        .collect()
}
