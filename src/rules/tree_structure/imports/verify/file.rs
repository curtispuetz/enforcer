use {
    super::rules,
    crate::{
        rules::{
            c::{files, path, use_paths},
            tree_structure::imports::t::{BadImport, Config, Fix, Violation},
        },
        t::Outcome,
    },
    std::{
        collections::{HashMap, HashSet},
        path::Path,
    },
};

type Found = (Vec<String>, &'static str, Option<Fix>);

pub fn run(path: &Path, config: &Config) -> Outcome<Violation> {
    if path::module(path).is_none() {
        return Outcome::Skipped;
    }
    let imports: Vec<BadImport> = _found(path, config)
        .into_iter()
        .map(|(use_path, reason, fix)| BadImport {
            text: format!("use {};", use_path.join("::")),
            reason,
            fix,
        })
        .collect();
    if imports.is_empty() {
        Outcome::Passed
    } else {
        Outcome::Failed(Violation {
            path: path::rel(path),
            imports,
        })
    }
}

pub fn fixes(path: &Path, config: &Config) -> (HashMap<Vec<String>, Vec<String>>, usize) {
    let mut rewrites = HashMap::new();
    let mut manual = 0;
    for (use_path, _, fix) in _found(path, config) {
        match fix {
            Some(Fix::Rewrite(new_path)) => {
                rewrites.insert(use_path, new_path);
            }
            _ => manual += 1,
        }
    }
    (rewrites, manual)
}

fn _found(path: &Path, config: &Config) -> Vec<Found> {
    let (Some(file_dir), Some(own_module)) = (path::file_dir(path), path::module(path))
    else {
        return Vec::new();
    };
    let mut found = Vec::new();
    for use_path in use_paths::internal(&files::ast_parse(path)) {
        if _is_ignored_macro(&use_path, config) {
            continue;
        }
        if let Some((reason, fix)) =
            rules::import_violation(&use_path, &file_dir, &own_module)
        {
            found.push((use_path, reason, fix));
        }
    }
    found
}

fn _is_ignored_macro(use_path: &[String], config: &Config) -> bool {
    config.ignore_exported_macros
        && _macro_is_exported(use_path, &config.exported_macros)
}

fn _macro_is_exported(use_path: &[String], exported_macros: &HashSet<String>) -> bool {
    use_path.len() == 2 && exported_macros.contains(&use_path[1])
}
