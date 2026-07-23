use colored::Colorize;

use crate::c::report;

use super::t::{Config, Violation};

pub fn print(config: Config, passed: usize, violations: Vec<Violation>) -> bool {
    report::summary(
        "file-sizes",
        passed,
        &format!(
            "All files under {} lines ({passed} files checked)",
            config.max_lines + 1
        ),
        violations,
        |violations| _print_failures(&config, violations),
    )
}

fn _print_failures(config: &Config, violations: Vec<Violation>) {
    println!(
        "The following file(s) have more than {} lines:\n",
        config.max_lines
    );
    for violation in &violations {
        println!(
            "  {} ({})",
            violation.path.bold(),
            format!("{} lines", violation.lines).red()
        );
    }
}
