use std::path::Path;

use crate::{
    c::{path, scan},
    t::{ItemsViolation, Outcome},
};

use super::{classify, issues, report};

pub fn run() -> bool {
    let (passed, violations) = scan::src_files(_check_file);
    report::print(passed, violations)
}

fn _check_file(path: &Path) -> Outcome<ItemsViolation> {
    if !classify::is_commons_root(path) {
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
