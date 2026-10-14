use {
    super::{
        report, suggest,
        t::{Config, Violation},
    },
    crate::{
        rules::{
            c::{files, path, scan},
            t::FileImports,
        },
        t::Outcome,
    },
    std::{cell::LazyCell, path::Path},
};

pub fn run() -> bool {
    scan::run_with_graph(Config::new(), _check_file, report::print)
}

fn _check_file(
    path: &Path,
    config: &Config,
    index: &LazyCell<Vec<FileImports>>,
) -> Outcome<Violation> {
    let relative = path::rel(path);
    if config.ignore.contains(&relative) {
        return Outcome::Skipped;
    }
    let lines = files::lines(path);
    if lines > config.max_lines {
        Outcome::Failed(Violation {
            path: relative,
            lines,
            fixes: suggest::fixes(path, index),
        })
    } else {
        Outcome::Passed
    }
}
