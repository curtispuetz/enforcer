use std::path::Path;

use crate::{
    rules::c::{outcome, path, scan},
    t::{ItemsViolation, Outcome},
};

use super::{issues, report};

pub fn run() -> bool {
    scan::run(_check_file, report::print)
}

fn _check_file(path: &Path) -> Outcome<ItemsViolation> {
    if path::is_mod_or_lib(path) {
        return Outcome::Skipped;
    }
    outcome::of_items(path, issues::of(path))
}
