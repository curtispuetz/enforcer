use {
    super::{
        describe,
        t::{Config, Violation},
    },
    crate::rules::{
        c::{report, suggestions},
        t::Results,
    },
    colored::Colorize,
};

const NO_FIX: &str = "no automatic move found; merge or regroup modules by hand";

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
    let mut texts: Vec<String> = violation.fixes.iter().map(describe::fix).collect();
    if texts.is_empty() {
        texts.push(NO_FIX.to_string());
    }
    texts.push(format!(
        "add \"{}\" to the [mod-count] ignore list in enforcer.toml, \
        if it's clean for this module to have many children",
        violation.module
    ));
    suggestions::print(&texts);
}
