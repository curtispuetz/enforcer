use crate::{
    rules::{c::report, t::Results},
    t::ItemsViolation,
};

const FIX: &str = "add '_' to the start of the names \
    (`cargo enforcer private-fn-naming --fix` does this automatically)";

pub fn print(res: Results<ItemsViolation>) -> bool {
    report::items(
        "fn-naming",
        res.passed,
        &format!(
            "All private functions/methods start with `_` ({} files checked)",
            res.passed
        ),
        "The following file(s) define private functions/methods without a leading underscore:",
        res.violations,
        &[FIX.to_string()],
    )
}
