use std::{collections::HashMap, path::Path};

use crate::{
    rules::{
        c::{files, path, scan},
        tree_structure::t::{FileViolation, PartReport},
    },
    t::{ItemsViolation, Outcome},
};

use super::{contents, home};

pub fn part() -> PartReport {
    let disallowed = contents::disallowed();
    let res = scan::src_files(|path| _check_file(path, &disallowed));
    PartReport {
        name: "common-items",
        unit: "files",
        passed: res.passed,
        violations: res.violations.into_iter().map(FileViolation::new).collect(),
    }
}

fn _check_file(
    path: &Path,
    disallowed: &HashMap<String, Vec<String>>,
) -> Outcome<ItemsViolation> {
    let rel = path::rel(path);
    let file = files::ast_parse(path);
    let mut items = home::misplaced(path, &file);
    items.extend(disallowed.get(&rel).cloned().unwrap_or_default());
    if items.is_empty() {
        Outcome::Passed
    } else {
        Outcome::Failed(ItemsViolation { path: rel, items })
    }
}
