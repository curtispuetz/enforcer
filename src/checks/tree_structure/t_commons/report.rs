use crate::{c::report, t::ItemsViolation};

pub fn print(passed: usize, violations: Vec<ItemsViolation>) -> bool {
    report::listed(
        "t-commons",
        passed,
        &format!(
            "All impls are located correctly and no t module calls an \
            outside free function ({passed} files checked)"
        ),
        "The following file(s) contain t-commons violations:",
        violations,
    )
}
