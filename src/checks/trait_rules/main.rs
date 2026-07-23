use std::path::Path;

use crate::{
    c::{files, path, scan},
    t::{ItemsViolation, Outcome},
};

use super::report;

pub fn run() -> bool {
    let (passed, violations) = scan::src_files(_check_file);
    report::print(passed, violations)
}

fn _check_file(path: &Path) -> Outcome<ItemsViolation> {
    if path::under_dir(path, "ext_traits")
        || path::commons_file_kind(path) == Some("ext_traits")
    {
        return Outcome::Skipped;
    }
    let items = _disallowed_traits(path);
    if items.is_empty() {
        Outcome::Passed
    } else {
        Outcome::Failed(ItemsViolation {
            path: path::rel(path),
            items,
        })
    }
}

fn _disallowed_traits(path: &Path) -> Vec<String> {
    let mut items = Vec::new();
    for item in files::parse(path).items {
        if let Some(desc) = _trait_def(&item) {
            items.push(desc);
        }
    }
    items
}

fn _trait_def(item: &syn::Item) -> Option<String> {
    match item {
        syn::Item::Trait(i) => Some(format!("trait {}", i.ident)),
        syn::Item::TraitAlias(i) => Some(format!("trait {}", i.ident)),
        _ => None,
    }
}
