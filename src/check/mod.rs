mod imports;
mod location;
mod rules;

use std::path::Path;

use crate::config::Config;
use crate::files::rs_files;
use crate::macros::is_exported_macro;
use crate::source::parse_file;
use imports::crate_use_paths;
use location::{file_dir_segments, is_static_or_types};
use rules::is_import_allowed;

pub fn check_dir(root: &Path, dir_name: &str, config: &Config) -> (usize, usize) {
    let mut passed = 0;
    let mut failed = 0;
    let dir = root.join(dir_name);
    for path in rs_files(root, dir_name) {
        if check_file(&path, &dir, config) > 0 {
            failed += 1;
        } else {
            passed += 1;
        }
    }
    (passed, failed)
}

fn check_file(path: &Path, src_dir: &Path, config: &Config) -> usize {
    let Some(file_dir) = file_dir_segments(path, src_dir) else {
        return 0;
    };
    if file_dir.iter().any(|s| is_static_or_types(s)) {
        return 0;
    }
    let violations = disallowed_imports(path, &file_dir, config);
    report_violations(path, &violations);
    violations.len()
}

fn disallowed_imports(path: &Path, file_dir: &[String], config: &Config) -> Vec<String> {
    let mut violations = Vec::new();
    for use_path in crate_use_paths(&parse_file(path)) {
        if is_ignored_macro(&use_path, config) {
            continue;
        }
        if !is_import_allowed(&use_path, file_dir) {
            violations.push(format!("  use {};", use_path.join("::")));
        }
    }
    violations
}

fn is_ignored_macro(use_path: &[String], config: &Config) -> bool {
    config.ignore_exported_macros && is_exported_macro(use_path, &config.exported_macros)
}

fn report_violations(path: &Path, violations: &[String]) {
    if violations.is_empty() {
        return;
    }
    eprintln!("{} has disallowed imports:", path.display());
    for v in violations {
        eprintln!("{v}");
    }
}
