use crate::{c::report, t::ItemsViolation};

pub fn print(passed: usize, violations: Vec<ItemsViolation>) -> bool {
    report::items(
        "c-mod-contents",
        passed,
        &format!(
            "All c/ mod.rs files follow the c module rules ({passed} files checked)"
        ),
        "The following c/ mod.rs file(s) violate the c module rules:",
        violations,
    )
}
