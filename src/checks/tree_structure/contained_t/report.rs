use crate::{c::report, t::ItemsViolation};

pub fn print(passed: usize, violations: Vec<ItemsViolation>) -> bool {
    report::listed(
        "contained-t",
        passed,
        &format!(
            "No t module calls a free function from another module ({passed} files checked)"
        ),
        "The following t module file(s) call outside free functions:",
        violations,
    )
}
