use colored::Colorize;

use crate::c::report;

use super::t::Violation;

pub fn print(passed: usize, violations: Vec<Violation>) -> bool {
    report::summary(
        "import-rules",
        passed,
        &format!("All files followed import-rules ({passed} files checked)"),
        violations,
        _print_failures,
    )
}

fn _print_failures(violations: Vec<Violation>) {
    for violation in &violations {
        println!("{} has disallowed imports:", violation.path.bold());
        for import in &violation.imports {
            println!("  {}", import.text.red());
            println!("    {}", import.reason.dimmed());
        }
    }
}
