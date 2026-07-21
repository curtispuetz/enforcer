use std::{collections::HashSet, path::Path};

use crate::{
    c::{files, path},
    import_rules::{
        check::{
            imports::crate_use_paths,
            location::{file_dir_segments, is_static_or_types},
            rules::is_import_allowed,
        },
        t::{config::Config, violation::Violation},
    },
};

pub fn run(path: &Path, config: &Config) -> Option<Violation> {
    let file_dir = file_dir_segments(path)?;
    if file_dir.iter().any(|s| is_static_or_types(s)) {
        return None;
    }
    let imports = _disallowed_imports(path, &file_dir, config);
    if imports.is_empty() {
        None
    } else {
        Some(Violation {
            path: path::rel(path),
            imports,
        })
    }
}

fn _disallowed_imports(
    path: &Path,
    file_dir: &[String],
    config: &Config,
) -> Vec<String> {
    let mut violations = Vec::new();
    for use_path in crate_use_paths(&files::parse(path)) {
        if _is_ignored_macro(&use_path, config) {
            continue;
        }
        if !is_import_allowed(&use_path, file_dir) {
            violations.push(format!("use {};", use_path.join("::")));
        }
    }
    violations
}

fn _is_ignored_macro(use_path: &[String], config: &Config) -> bool {
    config.ignore_exported_macros
        && _macro_is_exported(use_path, &config.exported_macros)
}

fn _macro_is_exported(use_path: &[String], exported_macros: &HashSet<String>) -> bool {
    use_path.len() == 2 && exported_macros.contains(&use_path[1])
}
