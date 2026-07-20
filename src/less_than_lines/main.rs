use std::{fs, path::Path};

use crate::{
    c::files,
    less_than_lines::{
        report::report,
        t::{config::Config, violation::Violation},
    },
    s::{EXISTING_SRC_DIRS, ROOT},
};

pub fn run() {
    let config = Config::new();
    let violations = _violations(&config);
    report(&config, &violations);
}

fn _violations(config: &Config) -> Vec<Violation> {
    let mut violations = Vec::new();
    for dir_name in EXISTING_SRC_DIRS.iter() {
        for path in files::rs(dir_name) {
            if let Some(violation) = _check_file(&path, config) {
                violations.push(violation);
            }
        }
    }
    violations
}

fn _check_file(path: &Path, config: &Config) -> Option<Violation> {
    let relative = _relative(path);
    if config.ignore.contains(&relative) {
        return None;
    }
    let lines = _line_count(path);
    if lines >= config.max_lines {
        Some(Violation {
            path: relative,
            lines,
        })
    } else {
        None
    }
}

fn _line_count(path: &Path) -> usize {
    fs::read_to_string(path)
        .map(|contents| contents.lines().count())
        .unwrap_or(0)
}

fn _relative(path: &Path) -> String {
    path.strip_prefix(ROOT.as_path())
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}
