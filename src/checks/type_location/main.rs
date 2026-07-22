use std::path::Path;

use crate::c::{ast, files, path, scan};

use super::{report, t::Violation};

pub fn run() -> bool {
    let (passed, violations) = scan::src_files(_check_file);
    report::print(passed, violations)
}

fn _check_file(path: &Path) -> Option<Violation> {
    if path::under_dir(path, "t") {
        return None;
    }
    let items = _misplaced_types(path);
    if items.is_empty() {
        None
    } else {
        Some(Violation {
            path: path::rel(path),
            items,
        })
    }
}

fn _misplaced_types(path: &Path) -> Vec<String> {
    let mut items = Vec::new();
    for item in files::parse(path).items {
        if let Some(desc) = _public_type(&item) {
            items.push(desc);
        }
    }
    items
}

fn _public_type(item: &syn::Item) -> Option<String> {
    match item {
        syn::Item::Struct(i) if ast::is_public(&i.vis) => {
            Some(format!("struct {}", i.ident))
        }
        syn::Item::Enum(i) if ast::is_public(&i.vis) => {
            Some(format!("enum {}", i.ident))
        }
        syn::Item::Trait(i) if ast::is_public(&i.vis) => {
            Some(format!("trait {}", i.ident))
        }
        syn::Item::Type(i) if ast::is_public(&i.vis) => {
            Some(format!("type {}", i.ident))
        }
        _ => None,
    }
}
