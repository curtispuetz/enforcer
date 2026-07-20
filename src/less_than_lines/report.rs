use std::process;

use crate::less_than_lines::t::{config::Config, violation::Violation};

pub fn report(config: &Config, violations: &[Violation]) {
    if !violations.is_empty() {
        _print_failures(config, violations);
        process::exit(1);
    }
    println!("All files under {} lines", config.max_lines);
}

fn _print_failures(config: &Config, violations: &[Violation]) {
    eprintln!(
        "\n{} file(s) have {} or more lines:\n",
        violations.len(),
        config.max_lines
    );
    for violation in violations {
        eprintln!("  {} ({} lines)", violation.path, violation.lines);
    }
}
