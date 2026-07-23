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
                "{} No t or s commons module is nested inside one of the same kind ({passed} modules checked)",
                *SUCCESS_TAG
            )
        },
        _print_failures,
    )
}

fn _print_failures(violations: Vec<Violation>) {
    println!(
        "The following commons module(s) are nested inside one of the same kind:\n"
    );
    for violation in &violations {
        println!(
            "  {} (nested inside {})",
            violation.path.bold(),
            violation.ancestor.red()
        );
    }
}
