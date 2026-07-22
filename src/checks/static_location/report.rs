use colored::Colorize;

use crate::{c::report, s::SUCCESS_TAG};

use super::t::Violation;

pub fn print(passed: usize, violations: Vec<Violation>) -> bool {
    report::summary(
        "static-location",
        passed,
        violations,
        || {
            println!(
                "{} All public statics are in s/ directories ({passed} files checked)",
                *SUCCESS_TAG
            )
        },
        _print_failures,
    )
}

fn _print_failures(violations: Vec<Violation>) {
    println!("The following file(s) define public statics outside an s/ directory:\n");
    for violation in &violations {
        println!(
            "  {} ({})",
            violation.path.bold(),
            violation.items.join(", ").red()
        );
    }
}
