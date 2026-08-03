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
    parts::from_files(
        "mod-location",
        "The following file(s) have mod statements but are not mod.rs or lib.rs:",
        _check_file,
    )
}

fn _check_file(path: &Path) -> Outcome<ItemsViolation> {
    if path::is_mod_or_lib(path) {
        return Outcome::Skipped;
    }
    outcome::of_items(path, _mod_decls(path))
}

fn _mod_decls(path: &Path) -> Vec<String> {
    let mut decls = Vec::new();
    for item in files::ast_parse(path).items {
        if let syn::Item::Mod(m) = item {
            decls.push(format!("mod {}", m.ident));
        }
    }
    decls
}
