use {
    super::{
        describe,
        t::{Config, Violation},
    },
    crate::rules::{c::report, t::Results},
    colored::Colorize,
};

const MAX_FIXES_SHOWN: usize = 5;

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
        let excess = violation.count - config.max;
        println!(
            "  {} ({})",
            violation.module.bold(),
            format!("{} modules; free {excess}", violation.count).red()
        );
        _print_fixes(violation);
    }
}

fn _print_fixes(violation: &Violation) {
    if violation.fixes.is_empty() {
        let none =
            "suggested fix: no automatic suggestion; merge or regroup modules by hand";
        println!("    {}", none.dimmed());
        return;
    }
    for fix in violation.fixes.iter().take(MAX_FIXES_SHOWN) {
        println!("    {}", describe::fix(fix).dimmed());
    }
    let hidden = violation.fixes.len().saturating_sub(MAX_FIXES_SHOWN);
    if hidden > 0 {
        println!("    {}", format!("... and {hidden} more").dimmed());
    }
}
