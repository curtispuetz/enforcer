use crate::c::ast;

pub fn violations(file: &syn::File) -> Vec<String> {
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
    if !_has_single_glob_use_c(&file.items) {
        issues.push("missing single `pub use c::*;` statement".to_string());
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

fn _has_single_glob_use_c(items: &[syn::Item]) -> bool {
    let mut uses = items.iter().filter_map(|item| match item {
        syn::Item::Use(u) => Some(u),
        _ => None,
    });
    matches!((uses.next(), uses.next()), (Some(u), None) if _is_glob_use_c(u))
}

fn _is_glob_use_c(u: &syn::ItemUse) -> bool {
    ast::is_public(&u.vis) && ast::glob_module(&u.tree).as_deref() == Some("c")
}
