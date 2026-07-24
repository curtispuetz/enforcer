use colored::Colorize;

use crate::{c::report, t::ItemsViolation};

pub fn print(passed: usize, violations: Vec<ItemsViolation>) -> bool {
    report::summary(
        "commons-items",
        passed,
        &format!(
            "All public types, statics, consts, and traits are in their commons modules ({passed} files checked)"
        ),
        violations,
        _print_failures,
    )
}

fn _print_failures(violations: Vec<ItemsViolation>) {
    println!(
        "The following file(s) define items outside the commons module that holds them:\n"
    );
    for violation in &violations {
        println!(
            "  {} ({})",
            violation.path.bold(),
            violation.items.join(", ").red()
        );
    }
}
