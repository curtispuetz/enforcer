use std::{collections::HashSet, path::Path};

use crate::{
    c::{files, path},
    checks::import_rules::t::{BadImport, Config, Violation},
};

use super::{
    imports::internal_use_paths,
    location::{file_dir_segments, own_module_segments},
    rules::import_violation,
};

pub fn run(path: &Path, config: &Config) -> Option<Violation> {
    let file_dir = file_dir_segments(path)?;
    let own_module = own_module_segments(path)?;
    let imports = _disallowed_imports(path, &file_dir, &own_module, config);
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
    own_module: &[String],
    config: &Config,
) -> Vec<BadImport> {
    let mut violations = Vec::new();
    for use_path in internal_use_paths(&files::parse(path)) {
        if _is_ignored_macro(&use_path, config) {
            continue;
        }
        if let Some(reason) = import_violation(&use_path, file_dir, own_module) {
            violations.push(BadImport {
                text: format!("use {};", use_path.join("::")),
                reason,
            });
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
