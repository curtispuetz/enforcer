use std::path::Path;

use crate::{
    c::{files, path, scan},
    t::{ItemsViolation, Outcome},
};

use super::{find, report};

pub fn run() -> bool {
    let (passed, violations) = scan::src_files(_check_file);
    report::print(passed, violations)
}

fn _check_file(path: &Path) -> Outcome<ItemsViolation> {
    let items = find::violations(&files::ast_parse(path));
    if items.is_empty() {
        Outcome::Passed
    } else {
        Outcome::Failed(ItemsViolation {
            path: path::rel(path),
            items,
        })
    }
}
