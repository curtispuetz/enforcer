use std::path::Path;

use crate::{
    rules::c::{files, outcome, scan},
    t::{ItemsViolation, Outcome},
};

use super::{c::names, report};

pub fn run() -> bool {
    scan::run(_check_file, report::print)
}

fn _check_file(path: &Path) -> Outcome<ItemsViolation> {
    outcome::of_items(path, _misnamed_fns(path))
}

fn _misnamed_fns(path: &Path) -> Vec<String> {
    let found = names::misnamed(&files::ast_parse(path));
    found
        .free
        .iter()
        .chain(found.methods.iter())
        .map(|name| format!("fn {name}"))
        .collect()
}
