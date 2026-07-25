use colored::Colorize;

use crate::checks::c::report;

use super::t::{Config, Violation};

use crate::checks::t::Results;

pub fn print(config: Config, res: Results<Violation>) -> bool {
    report::summary(
        "mod-count",
        res.passed,
        &format!(
            "All mod.rs and lib.rs files declare at most {} modules ({} files checked)",
            config.max, res.passed
        ),
        res.violations,
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
