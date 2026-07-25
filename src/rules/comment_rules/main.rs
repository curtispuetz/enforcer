use std::{fs, path::Path};

use crate::{
    rules::c::{path, scan},
    t::Outcome,
};

use super::{
    find, report, rules,
    t::{BadComment, Config, Violation},
};

pub fn run() -> bool {
    scan::run_with_config(Config::new(), _check_file, report::print)
}

fn _check_file(path: &Path, config: &Config) -> Outcome<Violation> {
    let bad = _bad_comments(path, config);
    if bad.is_empty() {
        Outcome::Passed
    } else {
        Outcome::Failed(Violation {
            path: path::rel(path),
            comments: bad,
        })
    }
}

fn _bad_comments(path: &Path, config: &Config) -> Vec<BadComment> {
    let source = fs::read_to_string(path).unwrap_or_default();
    let mut bad = Vec::new();
    for comment in find::comments(&source) {
        if let Some(violation) = rules::eval(&comment, config) {
            bad.push(violation);
        }
    }
    bad
}
