use {
    crate::{
        rules::{c::report, t::Results},
        t::ItemsViolation,
    },
    colored::Colorize,
};

pub fn print(res: Results<ItemsViolation>) -> bool {
    _listed(
        "calls",
        res.passed,
        &format!(
            "All function calls go through a parent module and never repeat a word \
            ({} files checked)",
            res.passed
        ),
        "The following file(s) call functions incorrectly:",
        res.violations,
    )
}

fn _listed(
    name: &str,
    passed: usize,
    success_msg: &str,
    header: &'static str,
    violations: Vec<ItemsViolation>,
) -> bool {
    report::summary(name, passed, success_msg, violations, |v| {
        _print_listed(header, v)
    })
}

fn _print_listed(header: &str, violations: Vec<ItemsViolation>) {
    println!("{header}\n");
    for violation in &violations {
        println!("  {}", violation.path.bold());
        for item in &violation.items {
            println!("    {}", item.red());
        }
    }
}
