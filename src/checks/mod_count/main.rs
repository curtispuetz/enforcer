use std::path::Path;

use crate::{
    checks::c::{files, path, scan},
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
    if !path::is_mod_or_lib(path) {
        return Outcome::Skipped;
    }
    let module = path::rel(path.parent().unwrap_or(path));
    if config.ignore.contains(&module) {
        return Outcome::Skipped;
    }
    let count = _mod_count(path);
    if count > config.max {
        Outcome::Failed(Violation { module, count })
    } else {
        Outcome::Passed
    }
}

fn _mod_count(path: &Path) -> usize {
    let mut count = 0;
    for item in files::ast_parse(path).items {
        if matches!(item, syn::Item::Mod(_)) {
            count += 1;
        }
    }
    count
}
