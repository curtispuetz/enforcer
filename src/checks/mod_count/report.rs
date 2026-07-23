use colored::Colorize;

use crate::c::report;

use super::t::{Config, Violation};

pub fn print(config: Config, passed: usize, violations: Vec<Violation>) -> bool {
    report::summary(
        "mod-count",
        passed,
        &format!("All mod.rs and lib.rs files declare at most {} modules ({passed} files checked)", config.max),
        violations,
        |violations| _print_failures(&config, violations),
    )
}

fn _print_failures(config: &Config, violations: Vec<Violation>) {
    println!(
        "The following module(s) declare more than {} modules:\n",
        config.max
    );
    for violation in &violations {
        println!(
            "  {} ({})",
            violation.module.bold(),
            format!("{} modules", violation.count).red()
        );
    }
}
