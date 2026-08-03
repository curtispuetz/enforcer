use {
    super::{imports, rules},
    crate::{
        rules::{
            c::{files, path},
            tree_structure::{
                c::path as path2,
                imports::t::{BadImport, Config, Violation},
            },
        },
        t::Outcome,
    },
    std::{collections::HashSet, path::Path},
};

pub fn run(path: &Path, config: &Config) -> Outcome<Violation> {
    let (Some(file_dir), Some(own_module)) =
        (path2::file_dir(path), path2::module(path))
    else {
        return Outcome::Skipped;
    };
    let imports = _disallowed_imports(path, &file_dir, &own_module, config);
    if imports.is_empty() {
        Outcome::Passed
    } else {
        Outcome::Failed(Violation {
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
    for use_path in imports::internal_use_paths(&files::ast_parse(path)) {
        if _is_ignored_macro(&use_path, config) {
            continue;
        }
        if let Some(reason) = rules::import_violation(&use_path, file_dir, own_module) {
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
