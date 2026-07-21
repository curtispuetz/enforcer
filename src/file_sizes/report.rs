use crate::file_sizes::t::{config::Config, violation::Violation};

pub fn report(config: Config, passed: usize, violations: Vec<Violation>) -> bool {
    println!("file-sizes report:");
    if !violations.is_empty() {
        _print_failures(config, passed, violations);
        return false;
    }
    println!(
        "All files under {} lines ({passed} files checked)",
        config.max_lines
    );
    true
}

fn _print_failures(config: Config, passed: usize, violations: Vec<Violation>) {
    println!(
        "\n{passed} files passed, {} files failed\n",
        violations.len()
    );
    println!(
        "The following file(s) have {} or more lines:\n",
        config.max_lines
    );
    for violation in violations {
        println!("  {} ({} lines)", violation.path, violation.lines);
    }
}
