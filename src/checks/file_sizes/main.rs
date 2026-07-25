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
    scan::run_with_config(Config::new(), _check_file, report::print)
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
