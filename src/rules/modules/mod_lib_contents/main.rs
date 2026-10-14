use {
    crate::{
        rules::{
            c::{files, outcome, parts, path},
            t::PartReport,
        },
        t::{ItemsViolation, Outcome},
    },
    std::path::Path,
};

pub fn part() -> PartReport {
    parts::with_fixes(
        "mod-lib-contents",
        "The following mod.rs or lib.rs file(s) contain statements other than mod and use:",
        _check_file,
        |path, violation| _fix(path, violation.items.len()).into_iter().collect(),
    )
}

fn _check_file(path: &Path) -> Outcome<ItemsViolation> {
    if !path::is_mod_or_lib(path) {
        return Outcome::Skipped;
    }
    outcome::of_items(path, _disallowed_items(path))
}

fn _fix(path: &Path, count: usize) -> Option<String> {
    let name = path.file_name()?.to_str()?;
    let them = if count == 1 { "it" } else { "them" };
    let home = if name == "lib.rs" && path.with_file_name("main.rs").is_file() {
        "a new sibling module"
    } else {
        "a sibling main.rs"
    };
    Some(format!(
        "move {them} into {home}, and pub use what's needed from {name}"
    ))
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
