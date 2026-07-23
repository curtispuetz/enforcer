use colored::Colorize;

use crate::{c::report, t::ItemsViolation};

pub fn print(passed: usize, violations: Vec<ItemsViolation>) -> bool {
    report::summary(
        "trait-rules",
        passed,
        &format!("All traits are extension traits in ext_traits modules ({passed} files checked)"),
        violations,
        _print_failures,
    )
}

fn _print_failures(violations: Vec<ItemsViolation>) {
    println!("The following file(s) define traits outside an ext_traits module:\n");
    for violation in &violations {
        println!(
            "  {} ({})",
            violation.path.bold(),
            violation.items.join(", ").red()
        );
    }
}
