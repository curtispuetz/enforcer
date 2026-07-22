use colored::Colorize;

use crate::{c::report, s::SUCCESS_TAG, t::ItemsViolation};

pub fn print(passed: usize, violations: Vec<ItemsViolation>) -> bool {
    report::summary(
        "commons-exports",
        passed,
        violations,
        || {
            println!(
                "{} All t/ and s/ root mod.rs files glob re-export their modules and \
                keep them private ({passed} files checked)",
                *SUCCESS_TAG
            )
        },
        _print_failures,
    )
}

fn _print_failures(violations: Vec<ItemsViolation>) {
    println!("The following t/ or s/ mod.rs file(s) violate the export rules:\n");
    for violation in &violations {
        println!(
            "  {} ({})",
            violation.path.bold(),
            violation.items.join(", ").red()
        );
    }
}
