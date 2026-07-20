mod c;
mod check;
mod macros;
mod source;
mod t;

use std::{
    collections::HashSet,
    path::{Path, PathBuf},
    process,
};

use t::config::Config;

const SOURCE_DIRS: [&str; 2] = ["src", "tests"];

fn main() {
    match subcommand().as_deref() {
        Some("import-rules") => import_rules(),
        Some(other) => {
            eprintln!("enforcer: unknown check '{other}'");
            eprintln!("available checks: import-rules");
            process::exit(2);
        }
        None => {
            eprintln!("enforcer: no check specified");
            eprintln!("usage: cargo enforcer <check>");
            eprintln!("available checks: import-rules");
            process::exit(2);
        }
    }
}

/// Returns the requested check name. When invoked as `cargo enforcer <check>`,
/// cargo injects a leading `enforcer` argument, which we skip.
fn subcommand() -> Option<String> {
    let mut positional = std::env::args().skip(1).filter(|a| !a.starts_with('-'));
    match positional.next() {
        Some(arg) if arg == "enforcer" => positional.next(),
        other => other,
    }
}

fn import_rules() {
    let root = manifest_dir().unwrap_or_else(|| PathBuf::from("."));
    let config = build_config(&root);
    let (passed, failed) = check_all(&root, &config);
    report(passed, failed);
}

fn build_config(root: &Path) -> Config {
    let ignore_exported_macros =
        std::env::args().any(|a| a == "--ignore-exported-macros");
    let mut exported_macros = HashSet::new();
    if ignore_exported_macros {
        for dir_name in existing_source_dirs(root) {
            exported_macros.extend(macros::collect_exported_macros(root, dir_name));
        }
    }
    Config {
        ignore_exported_macros,
        exported_macros,
    }
}

fn check_all(root: &Path, config: &Config) -> (usize, usize) {
    let mut passed = 0;
    let mut failed = 0;
    for dir_name in existing_source_dirs(root) {
        let (p, f) = check::dir(root, dir_name, config);
        passed += p;
        failed += f;
    }
    (passed, failed)
}

fn existing_source_dirs(root: &Path) -> impl Iterator<Item = &'static str> + '_ {
    SOURCE_DIRS
        .into_iter()
        .filter(|dir_name| root.join(dir_name).exists())
}

fn report(passed: usize, failed: usize) {
    if failed > 0 {
        print_failures(passed, failed);
        process::exit(1);
    }
    println!("All files good ({passed} files checked)");
}

fn print_failures(passed: usize, failed: usize) {
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

fn manifest_dir() -> Option<PathBuf> {
    let mut dir = std::env::current_dir().ok()?;
    loop {
        if dir.join("Cargo.toml").exists() {
            return Some(dir);
        }
        dir = dir.parent()?.to_path_buf();
    }
}
