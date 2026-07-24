use crate::{c::report, t::ItemsViolation};

pub fn print(passed: usize, violations: Vec<ItemsViolation>) -> bool {
    report::items(
        "fn-naming",
        passed,
        &format!(
            "All private functions/methods start with `_` ({passed} files checked)"
        ),
        "The following file(s) define private functions/methods without a leading underscore:",
        violations,
    )
}
