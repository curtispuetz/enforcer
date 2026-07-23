use colored::Colorize;

use crate::{c::report, t::ItemsViolation};

pub fn print(passed: usize, violations: Vec<ItemsViolation>) -> bool {
    report::summary(
        "type-location",
        passed,
        &format!(
            "All public structs, enums, traits, and type aliases are in t/ directories ({passed} files checked)"
        ),
        violations,
        _print_failures,
    )
}

fn _print_failures(violations: Vec<ItemsViolation>) {
    println!("The following file(s) define public types outside a t/ directory:\n");
    for violation in &violations {
        println!(
            "  {} ({})",
            violation.path.bold(),
            violation.items.join(", ").red()
        );
    }
}
