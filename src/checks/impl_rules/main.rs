use std::collections::{HashMap, HashSet};
use std::path::Path;

use crate::{
    c::{files, path, scan},
    t::{ItemsViolation, Outcome},
};

use super::{find, registry, report};

pub fn run() -> bool {
    let (public_defs, crate_types) = registry::build();
    let (passed, violations) =
        scan::src_files(|path| _check_file(path, &public_defs, &crate_types));
    report::print(passed, violations)
}

fn _check_file(
    path: &Path,
    public_defs: &HashMap<String, Vec<String>>,
    crate_types: &HashSet<String>,
) -> Outcome<ItemsViolation> {
    let items = find::violations(path, &files::parse(path), public_defs, crate_types);
    if items.is_empty() {
        Outcome::Passed
    } else {
        Outcome::Failed(ItemsViolation {
            path: path::rel(path),
            items,
        })
    }
}
