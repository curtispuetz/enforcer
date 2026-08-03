use {
    super::nesting,
    crate::rules::tree_structure::t::{FileViolation, PartReport},
};

pub fn part() -> PartReport {
    let res = nesting::run();
    PartReport {
        name: "common-nesting",
        unit: "rules",
        passed: res.passed,
        violations: res.violations.into_iter().map(FileViolation::new).collect(),
    }
}
