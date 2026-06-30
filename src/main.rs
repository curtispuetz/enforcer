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
        println!("{} files passed, {} files failed", total_passed, total_failed);
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
