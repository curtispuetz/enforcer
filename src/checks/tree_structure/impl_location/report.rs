use crate::{c::report, t::ItemsViolation};

pub fn print(passed: usize, violations: Vec<ItemsViolation>) -> bool {
    report::items(
        "impl-location",
        passed,
        &format!("All impls are located correctly ({passed} files checked)"),
        "The following file(s) contain misplaced impls:",
        violations,
    )
}
