use {
    super::t::{Config, Violation},
    crate::rules::{c::report, t::Results},
    colored::Colorize,
};

pub fn print(config: Config, res: Results<Violation>) -> bool {
    report::summary(
        "file-sizes",
        res.passed,
        &format!(
            "All files under {} lines ({} files checked)",
            config.max_lines + 1,
            res.passed
        ),
        res.violations,
        |violations| _print_failures(&config, violations),
    )
}

fn _print_failures(config: &Config, violations: Vec<Violation>) {
    println!(
        "The following file(s) have more than {} lines:\n",
        config.max_lines
    );
    for violation in &violations {
        println!(
            "  {} ({})",
            violation.path.bold(),
            format!("{} lines", violation.lines).red()
        );
    }
}
