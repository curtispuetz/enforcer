use crate::{c::report, t::ItemsViolation};

pub fn print(passed: usize, violations: Vec<ItemsViolation>) -> bool {
    report::items(
        "commons-items",
        passed,
        &format!(
            "All public items are defined in their commons module, and every commons module holds only its own kind ({passed} files checked)"
        ),
        "The following file(s) define items outside their commons module, or expose items their commons module does not hold:",
        violations,
    )
}
