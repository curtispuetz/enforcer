use colored::Colorize;

use crate::checks::{c::report, t::Results};

use super::t::{Config, Violation};

pub fn print(config: Config, res: Results<Violation>) -> bool {
    report::summary(
        "cognitive-complexity",
        res.passed,
        &format!(
            "All functions have cognitive complexity at most {} ({} files checked)",
            config.max, res.passed
        ),
        res.violations,
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
                format!("{}::{}", violation.path, function.name).red(),
                format!("[complexity {}]", function.score).dimmed()
            );
        }
    }
}
