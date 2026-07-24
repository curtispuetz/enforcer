use crate::{c::report, t::ItemsViolation};

pub fn print(passed: usize, violations: Vec<ItemsViolation>) -> bool {
    report::listed(
        "call-rules",
        passed,
        &format!(
            "All function calls go through a parent module and never repeat a word \
            ({passed} files checked)"
        ),
        "The following file(s) call functions incorrectly:",
        violations,
    )
}
