use crate::mod_location::t::violation::Violation;

pub fn report(passed: usize, violations: Vec<Violation>) -> bool {
    println!("mod-location report:");
    if !violations.is_empty() {
        _print_failures(passed, violations);
        return false;
    }
    println!(
        "All mod statements are in mod.rs or lib.rs files ({passed} files checked)"
    );
    true
}

fn _print_failures(passed: usize, violations: Vec<Violation>) {
    println!(
        "\n{passed} files passed, {} files failed\n",
        violations.len()
    );
    println!(
        "The following file(s) have mod statements but are not mod.rs or lib.rs:\n"
    );
    for violation in &violations {
        println!(
            "  {} (mod {})",
            violation.path,
            violation.mods.join(", mod ")
        );
    }
}
