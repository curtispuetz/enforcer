use colored::Colorize;

use crate::c::report;

use super::t::{Config, Violation};

pub fn print(config: Config, passed: usize, violations: Vec<Violation>) -> bool {
    report::summary(
        "cognitive-complexity",
        passed,
        &format!("All functions have cognitive complexity at most {} ({passed} files checked)", config.max),
        violations,
        |violations| _print_failures(&config, violations),
    )
}

fn _print_failures(config: &Config, violations: Vec<Violation>) {
    println!(
        "The following function(s) exceed the maximum cognitive complexity of {}:\n",
        config.max
    );
    for violation in &violations {
        for function in &violation.functions {
            println!(
                "  {}  {}  {}",
                format!("{}:{}", violation.path, function.line).bold(),
                function.name.red(),
                format!("[complexity {}]", function.score).dimmed()
            );
        }
    }
}
