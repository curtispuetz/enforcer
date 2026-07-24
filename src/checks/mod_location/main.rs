use std::path::Path;

use crate::{
    checks::c::{files, path, scan},
    t::Outcome,
};

use super::{report, t::Violation};

pub fn run() -> bool {
    scan::run(_check_file, report::print)
}

fn _check_file(path: &Path) -> Outcome<Violation> {
    if path::is_mod_or_lib(path) {
        return Outcome::Skipped;
    }
    let mods = _mod_names(path);
    if mods.is_empty() {
        Outcome::Passed
    } else {
        Outcome::Failed(Violation {
            path: path::rel(path),
            mods,
        })
    }
}

fn _mod_names(path: &Path) -> Vec<String> {
    let mut names = Vec::new();
    for item in files::ast_parse(path).items {
        if let syn::Item::Mod(m) = item {
            names.push(m.ident.to_string());
        }
    }
    names
}
