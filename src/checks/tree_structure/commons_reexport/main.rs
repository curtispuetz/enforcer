use crate::t::ItemsViolation;

use super::{c_contents, exports, nesting, report};

pub fn run() -> bool {
    let mut passed = 0;
    let mut violations: Vec<ItemsViolation> = Vec::new();
    for (part_passed, part_violations) in
        [nesting::check(), exports::check(), c_contents::check()]
    {
        passed += part_passed;
        violations.extend(part_violations);
    }
    report::print(passed, violations)
}
