use colored::Colorize;

use crate::{c::report, t::ItemsViolation};

pub fn print(passed: usize, violations: Vec<ItemsViolation>) -> bool {
    report::summary(
        "impl-rules",
        passed,
        &format!(
            "All impls sit beside their type, foreign-trait impls are in ext_traits, and t modules reach no crate free functions ({passed} files checked)"
        ),
        violations,
        _print_failures,
    )
}

fn _print_failures(violations: Vec<ItemsViolation>) {
    println!("The following file(s) break the impl rules:\n");
    for violation in &violations {
        println!("  {}", violation.path.bold());
        for item in &violation.items {
            println!("    {}", item.red());
        }
    }
}
