use std::path::Path;

use crate::{
    c::{ast, files, path, scan},
    t::{ItemsViolation, Outcome},
};

use super::report;

pub fn run() -> bool {
    let (passed, violations) = scan::src_files(_check_file);
    report::print(passed, violations)
}

fn _check_file(path: &Path) -> Outcome<ItemsViolation> {
    let items = _misnamed_fns(path);
    if items.is_empty() {
        Outcome::Passed
    } else {
        Outcome::Failed(ItemsViolation {
            path: path::rel(path),
            items,
        })
    }
}

fn _misnamed_fns(path: &Path) -> Vec<String> {
    let mut items = Vec::new();
    for item in &files::ast_parse(path).items {
        match item {
            syn::Item::Fn(f) => _check_fn(&f.vis, &f.sig.ident, &mut items),
            syn::Item::Impl(imp) if imp.trait_.is_none() => {
                for impl_item in &imp.items {
                    if let syn::ImplItem::Fn(f) = impl_item {
                        _check_fn(&f.vis, &f.sig.ident, &mut items);
                    }
                }
            }
            _ => {}
        }
    }
    items
}

fn _check_fn(vis: &syn::Visibility, ident: &syn::Ident, items: &mut Vec<String>) {
    let name = ident.to_string();
    if !ast::is_public(vis) && name != "main" && !name.starts_with('_') {
        items.push(format!("fn {name}"));
    }
}
