use colored::Colorize;

use crate::{c::report, checks::t::ItemsViolation, s::SUCCESS_TAG};

pub fn print(passed: usize, violations: Vec<ItemsViolation>) -> bool {
    report::summary(
        "commons-exports",
        passed,
        violations,
        || {
            println!(
                "{} All t/ and s/ root mod.rs files re-export their public items and \
                keep modules private ({passed} files checked)",
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
