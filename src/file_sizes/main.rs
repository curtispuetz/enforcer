use std::{fs, path::Path};

use crate::{
    c::files,
    file_sizes::{
        report::report,
        t::{config::Config, violation::Violation},
    },
    s::{EXISTING_SRC_DIRS, ROOT},
};

pub fn run() -> bool {
    let config = Config::new();
    let (passed, violations) = _check_all(&config);
    report(config, passed, violations)
}

fn _check_all(config: &Config) -> (usize, Vec<Violation>) {
    let mut passed = 0;
    let mut violations = Vec::new();
    for dir_name in EXISTING_SRC_DIRS.iter() {
        for path in files::rs(dir_name) {
            match _check_file(&path, config) {
                Some(violation) => violations.push(violation),
                None => passed += 1,
            }
        }
    }
    (passed, violations)
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
