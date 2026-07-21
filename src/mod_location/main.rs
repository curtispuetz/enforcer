use std::path::Path;

use crate::{
    c::{files, path, scan},
    mod_location::{report::report, t::violation::Violation},
};

pub fn run() -> bool {
    let (passed, violations) = scan::src_files(_check_file);
    report(passed, violations)
}

fn _check_file(path: &Path) -> Option<Violation> {
    if _is_mod_or_lib(path) {
        return None;
    }
    let mods = _mod_names(path);
    if mods.is_empty() {
        None
    } else {
        Some(Violation {
            path: path::rel(path),
            mods,
        })
    }
}

fn _is_mod_or_lib(path: &Path) -> bool {
    matches!(
        path.file_name().and_then(|n| n.to_str()),
        Some("mod.rs" | "lib.rs")
    )
}

fn _mod_names(path: &Path) -> Vec<String> {
    let mut names = Vec::new();
    for item in files::parse(path).items {
        if let syn::Item::Mod(m) = item {
            names.push(m.ident.to_string());
        }
    }
    names
}
