use crate::{
    c::report,
    comment_rules::t::{config::Config, violation::Violation},
};

pub fn report(config: Config, passed: usize, violations: Vec<Violation>) -> bool {
    report::summary(
        "comment-rules",
        passed,
        violations,
        || println!("[success] No disallowed comments ({passed} files checked)"),
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
                "  {}:{}  {}  [{}]",
                violation.path, comment.line, comment.text, comment.reason
            );
        }
    }
}
