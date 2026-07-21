use std::path::Path;

use crate::c::path;
use crate::{
    c::files,
    mod_location::{report::report, t::violation::Violation},
    s::EXISTING_SRC_DIRS,
};

pub fn run() -> bool {
    let (passed, violations) = _check_all();
    report(passed, violations)
}

fn _check_all() -> (usize, Vec<Violation>) {
    let mut passed = 0;
    let mut violations = Vec::new();
    for dir_name in EXISTING_SRC_DIRS.iter() {
        for path in files::rs(dir_name) {
            match _check_file(&path) {
                Some(violation) => violations.push(violation),
                None => passed += 1,
            }
        }
    }
    (passed, violations)
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
