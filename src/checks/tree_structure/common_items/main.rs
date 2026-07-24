use std::path::Path;

use crate::{
    checks::{
        c::{files, path, scan},
        tree_structure::t::{FileViolation, PartReport},
    },
    t::{ItemsViolation, Outcome},
};

use super::{contents, home};

pub fn part() -> PartReport {
    let (passed, violations) = scan::src_files(_check_file);
    PartReport {
        name: "common-items",
        unit: "files",
        passed,
        violations: violations.into_iter().map(FileViolation::new).collect(),
    }
}

fn _check_file(path: &Path) -> Outcome<ItemsViolation> {
    let file = files::ast_parse(path);
    let mut items = home::misplaced(path, &file);
    items.extend(contents::disallowed(path, &file));
    if items.is_empty() {
        Outcome::Passed
    } else {
        Outcome::Failed(ItemsViolation {
            path: path::rel(path),
            items,
        })
    }
}
