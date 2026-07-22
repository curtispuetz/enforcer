use std::path::Path;

use crate::c::{files, path, scan};

use super::{
    report,
    t::{Config, Violation},
};

pub fn run() -> bool {
    let config = Config::new();
    let (passed, violations) = scan::src_files(|path| _check_file(path, &config));
    report::print(config, passed, violations)
}

fn _check_file(path: &Path, config: &Config) -> Option<Violation> {
    if !path::is_mod_or_lib(path) {
        return None;
    }
    let module = path::rel(path.parent().unwrap_or(path));
    if config.ignore.contains(&module) {
        return None;
    }
    let count = _mod_count(path);
    if count > config.max {
        Some(Violation { module, count })
    } else {
        None
    }
}

fn _mod_count(path: &Path) -> usize {
    let mut count = 0;
    for item in files::parse(path).items {
        if matches!(item, syn::Item::Mod(_)) {
            count += 1;
        }
    }
    count
}
