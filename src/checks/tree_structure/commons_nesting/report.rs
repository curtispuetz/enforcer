use colored::Colorize;

use crate::c::report;

use super::t::Violation;

pub fn print(passed: usize, violations: Vec<Violation>) -> bool {
    report::summary(
        "commons-nesting",
        passed,
        &format!(
            "No commons module is nested inside another, except c inside c ({passed} modules checked)"
        ),
        violations,
        _print_failures,
    )
}

fn _print_failures(violations: Vec<Violation>) {
    println!(
        "The following commons module(s) are nested inside another commons module:\n"
    );
    for violation in &violations {
        println!(
            "  {} (nested inside {})",
            violation.path.bold(),
            violation.ancestor.red()
        );
    }
}
