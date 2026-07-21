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
    println!();
    _print_rules();
}

fn _print_rules() {
    println!(
        "For failures, they are somehow breaking the file structure rules (which \
        checked for both src/ and test/ files): "
    );
    println!("  1. a file can only import from its own directory or sub directories");
    println!(
        "  2. Exception to rule 1 is that files can import from c directories \
        if the prefix of that c directory path is in the files prefix"
    );
    println!(
        "  3. Rules 1 and 2 also apply within a c directory as if that c \
        directory were its own root directory project."
    );
    println!(
        "  4. The rules don't apply inside 's' (static data), 't' (types), \
        or 'ext_traits' directories, nor when any other file imports from \
        these directories."
    );
}
