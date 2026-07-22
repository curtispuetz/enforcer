use std::path::Path;

use crate::{
    c::{files, path, scan},
    t::Outcome,
};

use super::{
    measure, report,
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
    let mut over = Vec::new();
    for function in measure::functions(&files::parse(path)) {
        if function.score > config.max {
            over.push(function);
        }
    }
    if over.is_empty() {
        Outcome::Passed
    } else {
        Outcome::Failed(Violation {
            path: relative,
            functions: over,
        })
    }
}
