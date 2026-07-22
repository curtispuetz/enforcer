use std::path::Path;

use crate::c::{files, path, scan};

use super::{report, t::Violation};

pub fn run() -> bool {
    let (passed, violations) = scan::src_files(_check_file);
    report::print(passed, violations)
}

fn _check_file(path: &Path) -> Option<Violation> {
    if path::is_mod_or_lib(path) {
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

fn _mod_names(path: &Path) -> Vec<String> {
    let mut names = Vec::new();
    for item in files::parse(path).items {
        if let syn::Item::Mod(m) = item {
            names.push(m.ident.to_string());
        }
    }
    names
}
