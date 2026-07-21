use crate::{
    c::report,
    file_sizes::t::{config::Config, violation::Violation},
};

pub fn report(config: Config, passed: usize, violations: Vec<Violation>) -> bool {
    report::summary(
        "file-sizes",
        passed,
        violations,
        || {
            println!(
                "[success]All files under {} lines ({passed} files checked)",
                config.max_lines
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
        println!("  {} ({} lines)", violation.path, violation.lines);
    }
}
