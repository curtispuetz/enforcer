use colored::Colorize;

use crate::c::report;

use super::t::{Config, Violation};

pub fn print(config: Config, passed: usize, violations: Vec<Violation>) -> bool {
    report::summary(
        "comment-rules",
        passed,
        &format!("No disallowed comments ({passed} files checked)"),
        violations,
        |violations| _print_failures(&config, violations),
    )
}

fn _print_failures(config: &Config, violations: Vec<Violation>) {
    println!(
        "The following comment(s) are disallowed (allow with a 'not-obvious: ' prefix, or a trailing comment up to {} characters):\n",
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
