use colored::Colorize;

use crate::{c::report, checks::t::ItemsViolation, s::SUCCESS_TAG};

pub fn print(passed: usize, violations: Vec<ItemsViolation>) -> bool {
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

fn _print_failures(violations: Vec<ItemsViolation>) {
    println!("The following file(s) call functions incorrectly:\n");
    for violation in &violations {
        println!("  {}", violation.path.bold());
        for item in &violation.items {
            println!("    {}", item.red());
        }
    }
}
