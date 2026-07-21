use std::{fs, path::Path};

use crate::c::{path, scan};

use super::{
    find::comments,
    report, rules,
    t::{
        config::Config,
        violation::{BadComment, Violation},
    },
};

pub fn run() -> bool {
    let config = Config::new();
    let (passed, violations) = scan::src_files(|path| _check_file(path, &config));
    report(config, passed, violations)
}

fn _check_file(path: &Path, config: &Config) -> Option<Violation> {
    let bad = _bad_comments(path, config);
    if bad.is_empty() {
        None
    } else {
        Some(Violation {
            path: path::rel(path),
            comments: bad,
        })
    }
}

fn _bad_comments(path: &Path, config: &Config) -> Vec<BadComment> {
    let source = fs::read_to_string(path).unwrap_or_default();
    let mut bad = Vec::new();
    for comment in comments(&source) {
        if let Some(violation) = rules::check(&comment, config) {
            bad.push(violation);
        }
    }
    bad
}
