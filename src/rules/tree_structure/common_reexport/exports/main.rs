use std::path::Path;

use crate::{
    rules::{
        c::{outcome, scan},
        t::Results,
        tree_structure::c::path as path2,
    },
    t::{ItemsViolation, Outcome},
};

use super::issues;

pub fn run() -> Results<ItemsViolation> {
    scan::src_files(_check_file)
}

fn _check_file(path: &Path) -> Outcome<ItemsViolation> {
    if !_is_common_mod(path) {
        return Outcome::Skipped;
    }
    outcome::of_items(path, issues::of(path))
}

fn _is_common_mod(path: &Path) -> bool {
    if path.file_name().and_then(|n| n.to_str()) != Some("mod.rs") {
        return false;
    }
    ["t", "s", "cnst", "ext_traits"]
        .iter()
        .any(|name| path2::under_dir(path, name))
}
