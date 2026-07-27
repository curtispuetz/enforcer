use colored::Colorize;

use crate::rules::{c::report, t::Results};

use super::t::{Config, Violation};

pub fn print(config: Config, res: Results<Violation>) -> bool {
    report::summary(
        "comments",
        res.passed,
        &format!("No disallowed comments ({} files checked)", res.passed),
        res.violations,
        |violations| _print_failures(&config, violations),
    )
}

fn _print_failures(config: &Config, violations: Vec<Violation>) {
    println!(
        "The following comment(s) are disallowed (allow with a 'not-obvious: ' \
        or 'TODO' prefix, or a trailing comment up to {} characters):\n",
        config.max_trailing_len
    );
    for violation in &violations {
        for comment in &violation.comments {
            println!(
                "  {}  {}  {}",
                format!("{}:{}", violation.path, comment.line).bold(),
                comment.text.red(),
                format!("[{}]", comment.reason).dimmed()
            );
        }
    }
}
