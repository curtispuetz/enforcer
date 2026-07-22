use colored::Colorize;

use crate::{c::report, s::SUCCESS_TAG};

use super::t::Violation;

pub fn print(passed: usize, violations: Vec<Violation>) -> bool {
    report::summary(
        "c-mod-contents",
        passed,
        violations,
        || {
            println!(
                "{} All c/ mod.rs files follow the c module rules ({passed} files checked)",
                *SUCCESS_TAG
            )
        },
        _print_failures,
    )
}

fn _print_failures(violations: Vec<Violation>) {
    println!("The following c/ mod.rs file(s) violate the c module rules:\n");
    for violation in &violations {
        println!(
            "  {} ({})",
            violation.path.bold(),
            violation.items.join(", ").red()
        );
    }
}
