use crate::{c::report, t::ItemsViolation};

pub fn print(passed: usize, violations: Vec<ItemsViolation>) -> bool {
    report::items(
        "commons-items",
        passed,
        &format!(
            "All public types, statics, consts, and traits are in their commons modules ({passed} files checked)"
        ),
        "The following file(s) define items outside the commons module that holds them:",
        violations,
    )
}
