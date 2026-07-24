use colored::Colorize;

use crate::t::ItemsViolation;

pub fn items(
    name: &str,
    passed: usize,
    success_msg: &str,
    header: &'static str,
    violations: Vec<ItemsViolation>,
) -> bool {
    summary(name, passed, success_msg, violations, |v| {
        _print_items(header, v)
    })
}

fn _print_items(header: &str, violations: Vec<ItemsViolation>) {
    println!("{header}\n");
    for violation in &violations {
        println!(
            "  {} ({})",
            violation.path.bold(),
            violation.items.join(", ").red()
        );
    }
}

pub fn summary<V>(
    name: &str,
    passed: usize,
    success_msg: &str,
    violations: Vec<V>,
    on_fail: impl FnOnce(Vec<V>),
) -> bool {
    println!("{}", format!("{name} report:").bold().cyan());
    if violations.is_empty() {
        let s = "[success]".green().bold();
        println!("{s} {success_msg}");
        return false;
    }
    let failed = violations.len();
    println!(
        "\n{}, {}\n",
        format!("{passed} files passed").green(),
        format!("{failed} files failed").red().bold()
    );
    on_fail(violations);
    true
}
