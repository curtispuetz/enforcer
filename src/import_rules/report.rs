use crate::import_rules::t::violation::Violation;

pub fn report(passed: usize, violations: Vec<Violation>) -> bool {
    println!("import-rules report:");
    if !violations.is_empty() {
        _print_failures(passed, violations);
        return false;
    }
    println!("All files good ({passed} files checked)");
    true
}

fn _print_failures(passed: usize, violations: Vec<Violation>) {
    println!(
        "\n{passed} files passed, {} files failed\n",
        violations.len()
    );
    for violation in &violations {
        println!("{} has disallowed imports:", violation.path);
        for import in &violation.imports {
            println!("  {import}");
        }
    }
}
