use colored::Colorize;

use crate::s::FILE_CONFIG;

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
    let failed = violations.len();
    counted(
        name,
        success_msg,
        &format!("{passed} files passed"),
        &format!("{failed} files failed"),
        violations,
        on_fail,
    )
}

pub fn counted<V>(
    name: &str,
    success_msg: &str,
    passed_msg: &str,
    failed_msg: &str,
    violations: Vec<V>,
    on_fail: impl FnOnce(Vec<V>),
) -> bool {
    if violations.is_empty() {
        if FILE_CONFIG.debug {
            println!("{}", format!("{name} report:").bold().cyan());
            let s = "[success]".green().bold();
            println!("{s} {success_msg}");
        }
        return false;
    }
    println!("{}", format!("{name} report:").bold().cyan());
    println!("\n{}, {}\n", passed_msg.green(), failed_msg.red().bold());
    on_fail(violations);
    true
}
