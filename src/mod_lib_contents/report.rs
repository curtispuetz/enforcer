use colored::Colorize;

use crate::{c::report, s::SUCCESS_TAG};

use super::t::violation::Violation;

pub fn report(passed: usize, violations: Vec<Violation>) -> bool {
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

fn _print_failures(violations: Vec<Violation>) {
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
