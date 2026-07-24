use crate::{c::report, t::ItemsViolation};

pub fn print(passed: usize, violations: Vec<ItemsViolation>) -> bool {
    report::items(
        "commons-exports",
        passed,
        &format!(
            "All t/ and s/ root mod.rs files glob re-export their modules and keep them private ({passed} files checked)"
        ),
        "The following t/ or s/ mod.rs file(s) violate the export rules:",
        violations,
    )
}
