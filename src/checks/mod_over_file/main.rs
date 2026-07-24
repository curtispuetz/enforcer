use std::path::Path;

use crate::{
    checks::c::{path, scan},
    t::Outcome,
};

use super::{report, t::Violation};

pub fn run() -> bool {
    scan::run(_check_file, report::print)
}

fn _check_file(path: &Path) -> Outcome<Violation> {
    if path::is_mod_or_lib(path) {
        return Outcome::Skipped;
    }
    if _has_sibling_dir(path) {
        Outcome::Failed(Violation {
            path: path::rel(path),
            module: _stem(path),
        })
    } else {
        Outcome::Passed
    }
}

fn _has_sibling_dir(path: &Path) -> bool {
    match (path.parent(), path.file_stem()) {
        (Some(parent), Some(stem)) => parent.join(stem).is_dir(),
        _ => false,
    }
}

fn _stem(path: &Path) -> String {
    path.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or_default()
        .to_string()
}
