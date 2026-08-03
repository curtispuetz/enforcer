use {
    super::nesting,
    crate::rules::t::{FileViolation, PartReport},
};

pub fn part() -> PartReport {
    let res = nesting::run();
    PartReport {
        name: "common-nesting",
        failures_header:
            "The following common module(s) are nested inside a common module of another kind:",
        unit: "rules",
        passed: res.passed,
        violations: res.violations.into_iter().map(FileViolation::new).collect(),
    }
}
