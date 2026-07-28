use std::path::Path;

use crate::{
    rules::{
        c::scan,
        tree_structure::t::{FileViolation, PartReport},
    },
    t::{ItemsViolation, Outcome},
};

pub fn from_files(
    name: &'static str,
    rule: impl FnMut(&Path) -> Outcome<ItemsViolation>,
) -> PartReport {
    let res = scan::src_files(rule);
    PartReport {
        name,
        unit: "files",
        passed: res.passed,
        violations: res.violations.into_iter().map(FileViolation::new).collect(),
    }
}
