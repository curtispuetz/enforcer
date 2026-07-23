use colored::Colorize;

use crate::{c::report, t::ItemsViolation};

pub fn print(passed: usize, violations: Vec<ItemsViolation>) -> bool {
    report::summary(
        "use-privacy",
        passed,
        &format!("All use statements outside mod.rs and lib.rs are private ({passed} files checked)"),
        violations,
        _print_failures,
    )
}

fn _print_failures(violations: Vec<ItemsViolation>) {
    println!("The following file(s) have a `pub use` outside mod.rs or lib.rs:\n");
    for violation in &violations {
        println!(
            "  {} ({})",
            violation.path.bold(),
            violation.items.join(", ").red()
        );
    }
}
