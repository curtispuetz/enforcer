use std::path::Path;

use crate::{
    checks::c::{files, outcome, scan},
    t::{ItemsViolation, Outcome},
};

use super::{find, report};

pub fn run() -> bool {
    scan::run(_check_file, report::print)
}

fn _check_file(path: &Path) -> Outcome<ItemsViolation> {
    outcome::of_items(path, find::violations(&files::ast_parse(path)))
}
