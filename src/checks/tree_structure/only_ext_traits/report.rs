use crate::{c::report, t::ItemsViolation};

pub fn print(passed: usize, violations: Vec<ItemsViolation>) -> bool {
    report::items(
        "only-ext-traits",
        passed,
        &format!(
            "All traits are defined in ext_traits and implemented only for external types ({passed} files checked)"
        ),
        "The following file(s) define or implement traits incorrectly:",
        violations,
    )
}
