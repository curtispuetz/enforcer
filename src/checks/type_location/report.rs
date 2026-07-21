use colored::Colorize;

use crate::{c::report, s::SUCCESS_TAG};

use super::t::violation::Violation;

pub fn report(passed: usize, violations: Vec<Violation>) -> bool {
    report::summary(
        "type-location",
        passed,
        violations,
        || {
            println!(
                "{} All public structs, enums, traits, and type aliases are in t/ directories ({passed} files checked)",
                *SUCCESS_TAG
            )
        },
        _print_failures,
    )
}

fn _print_failures(violations: Vec<Violation>) {
    println!("The following file(s) define public types outside a t/ directory:\n");
    for violation in &violations {
        println!(
            "  {} ({})",
            violation.path.bold(),
            violation.items.join(", ").red()
        );
    }
}
