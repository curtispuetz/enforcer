use std::path::Path;

use crate::{
    checks::c::{ast, files, outcome, scan},
    t::{ItemsViolation, Outcome},
};

use super::report;

pub fn run() -> bool {
    scan::run(_check_file, report::print)
}

fn _check_file(path: &Path) -> Outcome<ItemsViolation> {
    outcome::of_items(path, _misnamed_fns(path))
}

fn _misnamed_fns(path: &Path) -> Vec<String> {
    let mut items = Vec::new();
    for item in &files::ast_parse(path).items {
        _check_item(item, &mut items);
    }
    items
}

fn _check_item(item: &syn::Item, items: &mut Vec<String>) {
    match item {
        syn::Item::Fn(f) => _check_fn(&f.vis, &f.sig.ident, items),
        syn::Item::Impl(imp) if imp.trait_.is_none() => {
            for impl_item in &imp.items {
                if let syn::ImplItem::Fn(f) = impl_item {
                    _check_fn(&f.vis, &f.sig.ident, items);
                }
            }
        }
        _ => {}
    }
}

fn _check_fn(vis: &syn::Visibility, ident: &syn::Ident, items: &mut Vec<String>) {
    let name = ident.to_string();
    if !ast::is_public(vis) && name != "main" && !name.starts_with('_') {
        items.push(format!("fn {name}"));
    }
}
