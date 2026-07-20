use std::process;

pub fn report(passed: usize, failed: usize) {
    if failed > 0 {
        _print_failures(passed, failed);
        process::exit(1);
    }
    println!("All files good ({passed} files checked)");
}

fn _print_failures(passed: usize, failed: usize) {
    println!("\n{passed} files passed, {failed} files failed\n");
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
