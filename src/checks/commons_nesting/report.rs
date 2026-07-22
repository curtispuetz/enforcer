use colored::Colorize;

use crate::{c::report, s::SUCCESS_TAG};

use super::t::Violation;

pub fn print(passed: usize, violations: Vec<Violation>) -> bool {
    report::summary(
        "commons-nesting",
        passed,
        violations,
        || {
            println!(
                "{} No t/ or s/ directory is nested inside one of the same kind ({passed} directories checked)",
                *SUCCESS_TAG
            )
        },
        _print_failures,
    )
}

fn _print_failures(violations: Vec<Violation>) {
    println!("The following commons director(ies) are nested inside one of the same kind:\n");
    for violation in &violations {
        println!(
            "  {} (nested inside {})",
            violation.path.bold(),
            violation.ancestor.red()
        );
    }
}
