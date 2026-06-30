mod check;

use std::{path::PathBuf, process};

fn main() {
    let root = _find_manifest_dir().unwrap_or_else(|| PathBuf::from("."));
    let mut total_violations = 0;

    for dir_name in ["src", "tests"] {
        if root.join(dir_name).exists() {
            total_violations += check::check_dir(&root, dir_name);
        }
    }

    if total_violations > 0 {
        process::exit(1);
    }
    println!("All files good");
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
