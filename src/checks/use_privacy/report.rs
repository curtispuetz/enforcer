use colored::Colorize;

use crate::{c::report, s::SUCCESS_TAG, t::ItemsViolation};

pub fn print(passed: usize, violations: Vec<ItemsViolation>) -> bool {
    report::summary(
        "use-privacy",
        passed,
        violations,
        || {
            println!(
                "{} All use statements outside mod.rs and lib.rs are private \
                ({passed} files checked)",
                *SUCCESS_TAG
            )
        },
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
