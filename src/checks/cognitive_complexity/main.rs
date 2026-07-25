use std::path::Path;

use crate::{
    checks::c::{files, path, scan},
    t::Outcome,
};

use super::{
    measure, report,
    t::{Config, Violation},
};

pub fn run() -> bool {
    scan::run_with_config(Config::new(), _check_file, report::print)
}

fn _check_file(path: &Path, config: &Config) -> Outcome<Violation> {
    let relative = path::rel(path);
    let mut over = Vec::new();
    for function in measure::functions(&files::ast_parse(path)) {
        let qualified = format!("{relative}::{}", function.name);
        if function.score > config.max && !config.ignore.contains(&qualified) {
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
