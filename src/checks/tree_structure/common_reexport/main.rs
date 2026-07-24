use crate::{
    checks::tree_structure::t::{FileViolation, PartReport},
    t::ItemsViolation,
};

use super::{c_contents, exports, nesting};

pub fn part() -> PartReport {
    let mut passed = 0;
    let mut violations: Vec<ItemsViolation> = Vec::new();
    for (part_passed, part_violations) in
        [nesting::check(), exports::check(), c_contents::check()]
    {
        passed += part_passed;
        violations.extend(part_violations);
    }
    PartReport {
        name: "common-reexport",
        unit: "checks",
        passed,
        violations: violations.into_iter().map(FileViolation::new).collect(),
    }
}
