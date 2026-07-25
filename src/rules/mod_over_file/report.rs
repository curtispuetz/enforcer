use colored::Colorize;

use crate::rules::c::report;

use super::t::Violation;

use crate::rules::t::Results;

pub fn print(res: Results<Violation>) -> bool {
    report::summary(
        "mod-over-file",
        res.passed,
        &format!(
            "All folder modules use mod.rs ({} files checked)",
            res.passed
        ),
        res.violations,
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
