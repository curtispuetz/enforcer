use colored::Colorize;

use crate::{c::report, s::SUCCESS_TAG};

use super::t::Violation;

pub fn report(passed: usize, violations: Vec<Violation>) -> bool {
    report::summary(
        "mod-location",
        passed,
        violations,
        || {
            println!(
                "{} All mod statements are in mod.rs or lib.rs files ({passed} files checked)",
                *SUCCESS_TAG
            )
        },
        _print_failures,
    )
}

fn _print_failures(violations: Vec<Violation>) {
    println!(
        "The following file(s) have mod statements but are not mod.rs or lib.rs:\n"
    );
    for violation in &violations {
        println!(
            "  {} ({})",
            violation.path.bold(),
            format!("mod {}", violation.mods.join(", mod ")).red()
        );
    }
}
