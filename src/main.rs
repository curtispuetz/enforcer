mod check;

use std::{path::PathBuf, process};

fn main() {
    let root = _find_manifest_dir().unwrap_or_else(|| PathBuf::from("."));
    let mut total_passed = 0;
    let mut total_failed = 0;

    for dir_name in ["src", "tests"] {
        if root.join(dir_name).exists() {
            let (passed, failed) = check::check_dir(&root, dir_name);
            total_passed += passed;
            total_failed += failed;
        }
    }

    if total_failed > 0 {
        println!(
            "{} files passed, {} files failed",
            total_passed, total_failed
        );
        println!(
            "For failures, they are somehow breaking the file structure rules (which \
            checked for both src/ and test/ files): "
        );
        println!(
            "  1. a file can only import from its own directory or sub \
            directories"
        );
        println!(
            "  2. Exception to rule 1 is that files can import from c directories \
            if the prefix of that c directory path is in the files prefix"
        );
        println!(
            "  3. Rules 1 and 2 also apply within a c directory as if that c \
            directory were its own root directory project."
        );
        println!(
            "  4. The rules don't apply inside 's' (static data) and 't' (types) \
            directories, nor when any other file imports from these directories."
        );
        process::exit(1);
    }
    println!("All files good ({} files checked)", total_passed);
}

fn _find_manifest_dir() -> Option<PathBuf> {
    let mut dir = std::env::current_dir().ok()?;
    loop {
        if dir.join("Cargo.toml").exists() {
            return Some(dir);
        }
        dir = dir.parent()?.to_path_buf();
    }
}
