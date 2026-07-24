use crate::{c::report, t::ItemsViolation};

pub fn print(passed: usize, violations: Vec<ItemsViolation>) -> bool {
    report::items(
        "commons-contents",
        passed,
        &format!(
            "All commons modules hold only their own kind of public item ({passed} files checked)"
        ),
        "The following file(s) expose public items their commons module does not hold:",
        violations,
    )
}
