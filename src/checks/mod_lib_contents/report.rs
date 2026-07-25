use crate::{
    checks::{c::report, t::Results},
    t::ItemsViolation,
};
pub fn print(res: Results<ItemsViolation>) -> bool {
    report::items(
        "mod-lib-contents",
        res.passed,
        &format!(
            "All mod.rs and lib.rs files contain only mod and use statements ({} files checked)",
            res.passed
        ),
        "The following mod.rs or lib.rs file(s) contain statements other than mod and use:",
        res.violations,
    )
}
