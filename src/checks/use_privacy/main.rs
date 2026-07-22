use std::path::Path;

use crate::{
    c::{path, scan},
    t::ItemsViolation,
};

use super::{issues, report};

pub fn run() -> bool {
    let (passed, violations) = scan::src_files(_check_file);
    report::print(passed, violations)
}

fn _check_file(path: &Path) -> Option<ItemsViolation> {
    if path::is_mod_or_lib(path) {
        return None;
    }
    let items = issues::of(path);
    if items.is_empty() {
        None
    } else {
        Some(ItemsViolation {
            path: path::rel(path),
            items,
        })
    }
}
