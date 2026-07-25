use crate::rules::tree_structure::t::{FileViolation, PartReport};

use super::nesting;

pub fn part() -> PartReport {
    let res = nesting::run();
    PartReport {
        name: "common-nesting",
        unit: "rules",
        passed: res.passed,
        violations: res.violations.into_iter().map(FileViolation::new).collect(),
    }
}
