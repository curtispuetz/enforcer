use std::path::Path;

use crate::{
    c::path,
    c::scan,
    t::{ItemsViolation, Outcome},
};

use super::{items, kind, report};

pub fn run() -> bool {
    let (passed, violations) = scan::src_files(_check_file);
    report::print(passed, violations)
}

fn _check_file(path: &Path) -> Outcome<ItemsViolation> {
    let Some(commons) = kind::of(path) else {
        return Outcome::Skipped;
    };
    let items = items::disallowed(path, commons);
    if items.is_empty() {
        Outcome::Passed
    } else {
        Outcome::Failed(ItemsViolation {
            path: path::rel(path),
            items,
        })
    }
}
