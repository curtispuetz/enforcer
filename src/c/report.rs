use colored::Colorize;

pub fn summary<V>(
    name: &str,
    passed: usize,
    violations: Vec<V>,
    on_pass: impl FnOnce(),
    on_fail: impl FnOnce(Vec<V>),
) -> bool {
    println!("{}", format!("{name} report:").bold().cyan());
    if violations.is_empty() {
        on_pass();
        return true;
    }
    let failed = violations.len();
    println!(
        "\n{}, {}\n",
        format!("{passed} files passed").green(),
        format!("{failed} files failed").red().bold()
    );
    on_fail(violations);
    false
}
