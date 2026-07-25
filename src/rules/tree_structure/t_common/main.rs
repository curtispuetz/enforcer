use std::path::Path;

use crate::{
    rules::{
        c::{files, path, scan},
        tree_structure::t::{FileViolation, PartReport},
    },
    t::{ItemsViolation, Outcome},
};

use super::{defs, impls, t::Defs};

pub fn part() -> PartReport {
    let defs = defs::find();
    let res = scan::src_files(|path| _check_file(path, &defs));
    PartReport {
        name: "t-common",
        unit: "files",
        passed: res.passed,
        violations: res.violations.into_iter().map(FileViolation::new).collect(),
    }
}

fn _check_file(path: &Path, defs: &Defs) -> Outcome<ItemsViolation> {
    let file = files::ast_parse(path);
    let items = impls::misplaced(&file, path, defs);
    if items.is_empty() {
        Outcome::Passed
    } else {
        Outcome::Failed(ItemsViolation {
            path: path::rel(path),
            items,
        })
    }
}
