use colored::Colorize;

use crate::{c::report, t::ItemsViolation};

pub fn print(passed: usize, violations: Vec<ItemsViolation>) -> bool {
    report::summary(
        "contained-t",
        passed,
        &format!(
            "No t module calls a free function from another module ({passed} files checked)"
        ),
        violations,
        _print_failures,
    )
}

fn _print_failures(violations: Vec<ItemsViolation>) {
    println!("The following t module file(s) call outside free functions:\n");
    for violation in &violations {
        println!("  {}", violation.path.bold());
        for item in &violation.items {
            println!("    {}", item.red());
        }
    }
}
