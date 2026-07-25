use crate::{
    checks::{c::report, t::Results},
    t::ItemsViolation,
};

pub fn print(res: Results<ItemsViolation>) -> bool {
    report::items(
        "use-privacy",
        res.passed,
        &format!(
            "All use statements outside mod.rs and lib.rs are private ({} files checked)",
            res.passed
        ),
        "The following file(s) have a `pub use` outside mod.rs or lib.rs:",
        res.violations,
    )
}
