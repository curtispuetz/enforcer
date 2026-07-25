use crate::rules::{
    t::Results,
    tree_structure::t::{FileViolation, PartReport},
};

use super::{c_contents, exports, nesting};

pub fn part() -> PartReport {
    let mut res = Results::new();
    for part in [nesting::run(), exports::run(), c_contents::run()] {
        res.passed += part.passed;
        res.violations.extend(part.violations);
    }
    PartReport {
        name: "common-reexport",
        unit: "rules",
        passed: res.passed,
        violations: res.violations.into_iter().map(FileViolation::new).collect(),
    }
}
