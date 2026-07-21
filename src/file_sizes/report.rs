use colored::Colorize;

use crate::{
    c::report,
    file_sizes::t::{config::Config, violation::Violation},
    s::SUCCESS_TAG,
};

pub fn report(config: Config, passed: usize, violations: Vec<Violation>) -> bool {
    report::summary(
        "file-sizes",
        passed,
        violations,
        || {
            println!(
                "{} All files under {} lines ({passed} files checked)",
                *SUCCESS_TAG, config.max_lines
            )
        },
        |violations| _print_failures(&config, violations),
    )
}

fn _print_failures(config: &Config, violations: Vec<Violation>) {
    println!(
        "The following file(s) have {} or more lines:\n",
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
