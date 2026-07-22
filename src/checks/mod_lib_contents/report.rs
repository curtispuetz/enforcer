use colored::Colorize;

use crate::{c::report, s::SUCCESS_TAG, t::ItemsViolation};

pub fn print(passed: usize, violations: Vec<ItemsViolation>) -> bool {
    report::summary(
        "mod-lib-contents",
        passed,
        violations,
        || {
            println!(
                "{} All mod.rs and lib.rs files contain only mod and use statements ({passed} files checked)",
                *SUCCESS_TAG
            )
        },
        _print_failures,
    )
}

fn _print_failures(violations: Vec<ItemsViolation>) {
    println!(
        "The following mod.rs or lib.rs file(s) contain statements other than mod and use:\n"
    );
    for violation in &violations {
        println!(
            "  {} ({})",
            violation.path.bold(),
            violation.items.join(", ").red()
        );
    }
}
