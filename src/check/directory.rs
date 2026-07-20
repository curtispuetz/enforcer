use std::path::Path;

use crate::check::imports::crate_use_paths;
use crate::check::location::{file_dir_segments, is_static_or_types};
use crate::check::rules::is_import_allowed;
use crate::{
    c::files, macros::is_exported_macro, source::parse_file, t::config::Config,
};

pub fn dir(root: &Path, dir_name: &str, config: &Config) -> (usize, usize) {
    let mut passed = 0;
    let mut failed = 0;
    let dir = root.join(dir_name);
    for path in files::rs(root, dir_name) {
        if _check_file(&path, &dir, config) > 0 {
            failed += 1;
        } else {
            passed += 1;
        }
    }
    (passed, failed)
}

fn _check_file(path: &Path, src_dir: &Path, config: &Config) -> usize {
    let Some(file_dir) = file_dir_segments(path, src_dir) else {
        return 0;
    };
    if file_dir.iter().any(|s| is_static_or_types(s)) {
        return 0;
    }
    let violations = _disallowed_imports(path, &file_dir, config);
    _report_violations(path, &violations);
    violations.len()
}

fn _disallowed_imports(
    path: &Path,
    file_dir: &[String],
    config: &Config,
) -> Vec<String> {
    let mut violations = Vec::new();
    for use_path in crate_use_paths(&parse_file(path)) {
        if _is_ignored_macro(&use_path, config) {
            continue;
        }
        if !is_import_allowed(&use_path, file_dir) {
            violations.push(format!("  use {};", use_path.join("::")));
        }
    }
    violations
}

fn _is_ignored_macro(use_path: &[String], config: &Config) -> bool {
    config.ignore_exported_macros
        && is_exported_macro(use_path, &config.exported_macros)
}

fn _report_violations(path: &Path, violations: &[String]) {
    if violations.is_empty() {
        return;
    }
    eprintln!("{} has disallowed imports:", path.display());
    for v in violations {
        eprintln!("{v}");
    }
}
