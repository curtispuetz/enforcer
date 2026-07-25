use std::path::Path;

use crate::{
    rules::c::{files, outcome, path, scan},
    t::{ItemsViolation, Outcome},
};

use super::report;

pub fn run() -> bool {
    scan::run(_check_file, report::print)
}

fn _check_file(path: &Path) -> Outcome<ItemsViolation> {
    if !path::is_mod_or_lib(path) {
        return Outcome::Skipped;
    }
    outcome::of_items(path, _disallowed_items(path))
}

fn _disallowed_items(path: &Path) -> Vec<String> {
    let mut items = Vec::new();
    for item in files::ast_parse(path).items {
        if !matches!(item, syn::Item::Mod(_) | syn::Item::Use(_)) {
            items.push(_item_kind(&item));
        }
    }
    items
}

fn _item_kind(item: &syn::Item) -> String {
    match item {
        syn::Item::Const(_) => "const",
        syn::Item::Enum(_) => "enum",
        syn::Item::ExternCrate(_) => "extern crate",
        syn::Item::Fn(_) => "fn",
        syn::Item::ForeignMod(_) => "extern block",
        syn::Item::Impl(_) => "impl",
        syn::Item::Macro(_) => "macro",
        syn::Item::Static(_) => "static",
        syn::Item::Struct(_) => "struct",
        syn::Item::Trait(_) => "trait",
        syn::Item::TraitAlias(_) => "trait alias",
        syn::Item::Type(_) => "type alias",
        syn::Item::Union(_) => "union",
        _ => "item",
    }
    .to_string()
}
