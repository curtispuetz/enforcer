use crate::{c::report, t::ItemsViolation};

pub fn print(passed: usize, violations: Vec<ItemsViolation>) -> bool {
    report::listed(
        "commons-reexport",
        passed,
        &format!(
            "All commons modules respect nesting and re-export rules ({passed} checks passed)"
        ),
        "The following commons module(s) violate the nesting/re-export rules:",
        violations,
    )
}
