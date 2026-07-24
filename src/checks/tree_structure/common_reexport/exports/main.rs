use std::path::Path;

use crate::{
    c::{path, scan},
    t::{ItemsViolation, Outcome},
};

use super::issues;

pub fn check() -> (usize, Vec<ItemsViolation>) {
    scan::src_files(_check_file)
}

fn _check_file(path: &Path) -> Outcome<ItemsViolation> {
    if !_is_common_mod(path) {
        return Outcome::Skipped;
    }
    let items = issues::of(path);
    if items.is_empty() {
        Outcome::Passed
    } else {
        Outcome::Failed(ItemsViolation {
            path: path::rel(path),
            items,
        })
    }
}

fn _is_common_mod(path: &Path) -> bool {
    if path.file_name().and_then(|n| n.to_str()) != Some("mod.rs") {
        return false;
    }
    ["t", "s", "cnst", "ext_traits"]
        .iter()
        .any(|name| path::under_dir(path, name))
}
