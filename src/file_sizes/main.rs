use std::{fs, path::Path};

use crate::{
    c::{path, scan},
    file_sizes::{
        report::report,
        t::{config::Config, violation::Violation},
    },
};

pub fn run() -> bool {
    let config = Config::new();
    let (passed, violations) = scan::src_files(|path| _check_file(path, &config));
    report(config, passed, violations)
}

fn _check_file(path: &Path, config: &Config) -> Option<Violation> {
    let relative = path::rel(path);
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
