use std::path::Path;

use crate::c::ast;

use super::reexports;

pub fn violations(path: &Path, file: &syn::File) -> Vec<String> {
    let mut issues = Vec::new();
    match _private_mod_c(&file.items) {
        None => issues.push("missing `mod c;` declaration".to_string()),
        Some(m) if !_has_inception_allow(&m.attrs) => {
            issues.push(
                "`mod c;` missing `#[allow(clippy::module_inception)]`".to_string(),
            );
        }
        Some(_) => {}
    }
    match _single_pub_use_c(&file.items) {
        Some(u) => {
            for name in reexports::missing(u, path) {
                issues.push(format!("`pub use c` does not re-export `{name}`"));
            }
        }
        None => issues.push("missing single `pub use c::{...}` statement".to_string()),
    }
    for item in &file.items {
        if let syn::Item::Mod(m) = item
            && m.ident != "c"
            && !ast::is_public(&m.vis)
        {
            issues.push(format!("module `{}` is not public", m.ident));
        }
    }
    issues
}

fn _has_inception_allow(attrs: &[syn::Attribute]) -> bool {
    attrs.iter().any(_is_inception_allow)
}

fn _is_inception_allow(attr: &syn::Attribute) -> bool {
    if !matches!(attr.style, syn::AttrStyle::Outer) {
        return false;
    }
    let Ok(list) = attr.meta.require_list() else {
        return false;
    };
    list.path.is_ident("allow")
        && list.tokens.to_string().replace(' ', "") == "clippy::module_inception"
}

fn _private_mod_c(items: &[syn::Item]) -> Option<&syn::ItemMod> {
    items.iter().find_map(|item| match item {
        syn::Item::Mod(m)
            if m.ident == "c" && !ast::is_public(&m.vis) && m.content.is_none() =>
        {
            Some(m)
        }
        _ => None,
    })
}

fn _single_pub_use_c(items: &[syn::Item]) -> Option<&syn::ItemUse> {
    let mut uses = items.iter().filter_map(|item| match item {
        syn::Item::Use(u) => Some(u),
        _ => None,
    });
    match (uses.next(), uses.next()) {
        (Some(u), None) if _is_pub_use_c(u) => Some(u),
        _ => None,
    }
}

fn _is_pub_use_c(u: &syn::ItemUse) -> bool {
    ast::is_public(&u.vis) && matches!(&u.tree, syn::UseTree::Path(p) if p.ident == "c")
}
