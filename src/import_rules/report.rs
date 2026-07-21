use colored::Colorize;

use crate::{c::report, import_rules::t::violation::Violation, s::SUCCESS_TAG};

pub fn report(passed: usize, violations: Vec<Violation>) -> bool {
    report::summary(
        "import-rules",
        passed,
        violations,
        || {
            println!(
                "{} All files followed import-rules ({passed} files checked)",
                *SUCCESS_TAG
            )
        },
        _print_failures,
    )
}

fn _print_failures(violations: Vec<Violation>) {
    for violation in &violations {
        println!("{} has disallowed imports:", violation.path.bold());
        for import in &violation.imports {
            println!("  {}", import.red());
        }
    }
}
