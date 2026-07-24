use std::path::Path;

use crate::{
    c::{files, path, scan},
    checks::tree_structure::{c, t::PartReport},
    t::{ItemsViolation, Outcome},
};

use super::{contents, home};

pub fn part() -> PartReport {
    let (passed, violations) = scan::src_files(_check_file);
    PartReport {
        name: "common-items",
        unit: "files",
        passed,
        violations: violations.into_iter().map(c::file_violation).collect(),
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
