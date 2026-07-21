use colored::Colorize;

use crate::{c::report, s::SUCCESS_TAG};

use super::t::{config::Config, violation::Violation};

pub fn report(config: Config, passed: usize, violations: Vec<Violation>) -> bool {
    report::summary(
        "comment-rules",
        passed,
        violations,
        || {
            println!(
                "{} No disallowed comments ({passed} files checked)",
                *SUCCESS_TAG
            )
        },
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
