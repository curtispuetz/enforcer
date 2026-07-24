use std::{collections::HashSet, path::Path};

use crate::{
    c::{path, scan, type_names},
    t::{ItemsViolation, Outcome},
};

use super::{check, report};

pub fn run() -> bool {
    let crate_types = type_names::defined();
    let (passed, violations) = scan::src_files(|path| _check_file(path, &crate_types));
    report::print(passed, violations)
}

fn _check_file(path: &Path, crate_types: &HashSet<String>) -> Outcome<ItemsViolation> {
    let items = check::violations(path, crate_types);
    if items.is_empty() {
        Outcome::Passed
    } else {
        Outcome::Failed(ItemsViolation {
            path: path::rel(path),
            items,
        })
    }
}
