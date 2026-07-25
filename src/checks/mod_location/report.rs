use colored::Colorize;

use crate::checks::c::report;

use super::t::Violation;

use crate::checks::t::Results;

pub fn print(res: Results<Violation>) -> bool {
    report::summary(
        "mod-location",
        res.passed,
        &format!(
            "All mod statements are in mod.rs or lib.rs files ({} files checked)",
            res.passed
        ),
        res.violations,
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
