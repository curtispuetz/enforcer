use colored::Colorize;

use crate::{c::report, t::ItemsViolation};

pub fn print(passed: usize, violations: Vec<ItemsViolation>) -> bool {
    report::summary(
        "c-mod-contents",
        passed,
        &format!(
            "All c/ mod.rs files follow the c module rules ({passed} files checked)"
        ),
        violations,
        _print_failures,
    )
}

fn _print_failures(violations: Vec<ItemsViolation>) {
    println!("The following c/ mod.rs file(s) violate the c module rules:\n");
    for violation in &violations {
        println!(
            "  {} ({})",
            violation.path.bold(),
            violation.items.join(", ").red()
        );
    }
}
