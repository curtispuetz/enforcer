use colored::Colorize;

use crate::{c::report, s::SUCCESS_TAG};

use super::t::Violation;

pub fn report(passed: usize, violations: Vec<Violation>) -> bool {
    report::summary(
        "call-rules",
        passed,
        violations,
        || {
            println!(
                "{} All function calls go through a parent module and never repeat a word ({passed} files checked)",
                *SUCCESS_TAG
            )
        },
        _print_failures,
    )
}

fn _print_failures(violations: Vec<Violation>) {
    println!("The following file(s) call functions incorrectly:\n");
    for violation in &violations {
        println!("  {}", violation.path.bold());
        for item in &violation.items {
            println!("    {}", item.red());
        }
    }
}
