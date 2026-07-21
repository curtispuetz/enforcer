pub fn summary<V>(
    name: &str,
    passed: usize,
    violations: Vec<V>,
    on_pass: impl FnOnce(),
    on_fail: impl FnOnce(Vec<V>),
) -> bool {
    println!("{name} report:");
    if violations.is_empty() {
        on_pass();
        return true;
    }
    println!(
        "\n{passed} files passed, {} files failed\n",
        violations.len()
    );
    on_fail(violations);
    false
}
