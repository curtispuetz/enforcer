use crate::{checks::c::report, t::ItemsViolation};

pub fn print(passed: usize, violations: Vec<ItemsViolation>) -> bool {
    report::items(
        "mod-lib-contents",
        passed,
        &format!(
            "All mod.rs and lib.rs files contain only mod and use statements ({passed} files checked)"
        ),
        "The following mod.rs or lib.rs file(s) contain statements other than mod and use:",
        violations,
    )
}
