use std::{fs, path::Path};

use crate::{
    checks::c::{path, scan},
    t::Outcome,
};

use super::{
    report,
    t::{Config, Violation},
};

pub fn run() -> bool {
    let config = Config::new();
    let (passed, violations) = scan::src_files(|path| _check_file(path, &config));
    report::print(config, passed, violations)
}

fn _check_file(path: &Path, config: &Config) -> Outcome<Violation> {
    let relative = path::rel(path);
    if config.ignore.contains(&relative) {
        return Outcome::Skipped;
    }
    let lines = _line_count(path);
    if lines > config.max_lines {
        Outcome::Failed(Violation {
            path: relative,
            lines,
        })
    } else {
        Outcome::Passed
    }
}

fn _line_count(path: &Path) -> usize {
    fs::read_to_string(path)
        .map(|contents| contents.lines().count())
        .unwrap_or(0)
}
