use {
    super::t::{Config, Violation},
    crate::rules::{
        c::{report, suggestions},
        t::Results,
    },
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
        _print_fixes(violation);
    }
}

fn _print_fixes(violation: &Violation) {
    let mut texts = violation.fixes.clone();
    texts.push(format!(
        "add \"{}\" to the [file-sizes] ignore list in enforcer.toml, \
        if the file is simple and just holds a lot of data",
        violation.path
    ));
    suggestions::print(&texts);
}
