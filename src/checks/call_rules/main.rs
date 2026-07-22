use std::path::Path;

use crate::{
    c::{files, path, scan},
    t::ItemsViolation,
};

use super::{find, report};

pub fn run() -> bool {
    let (passed, violations) = scan::src_files(_check_file);
    report::print(passed, violations)
}

fn _check_file(path: &Path) -> Option<ItemsViolation> {
    let items = find::violations(&files::parse(path));
    if items.is_empty() {
        None
    } else {
        Some(ItemsViolation {
            path: path::rel(path),
            items,
        })
    }
}
