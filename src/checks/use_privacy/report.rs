use crate::{c::report, t::ItemsViolation};

pub fn print(passed: usize, violations: Vec<ItemsViolation>) -> bool {
    report::items(
        "use-privacy",
        passed,
        &format!(
            "All use statements outside mod.rs and lib.rs are private ({passed} files checked)"
        ),
        "The following file(s) have a `pub use` outside mod.rs or lib.rs:",
        violations,
    )
}
