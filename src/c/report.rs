use colored::Colorize;

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
