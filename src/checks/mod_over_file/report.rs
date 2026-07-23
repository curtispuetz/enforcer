use colored::Colorize;

use crate::c::report;

use super::t::Violation;

pub fn print(passed: usize, violations: Vec<Violation>) -> bool {
    report::summary(
        "mod-over-file",
        passed,
        &format!("All folder modules use mod.rs ({passed} files checked)"),
        violations,
        _print_failures,
    )
}

fn _print_failures(violations: Vec<Violation>) {
    println!(
        "The following file(s) shadow a sibling folder and should become that folder's mod.rs:\n"
    );
    for violation in &violations {
        println!(
            "  {} (move into {})",
            violation.path.bold(),
            format!("{}/mod.rs", violation.module).red()
        );
    }
}
