use std::path::Path;

use crate::c::{path, scan};

use super::{report, t::Violation};

pub fn run() -> bool {
    let (passed, violations) = scan::src_files(_check_file);
    report::print(passed, violations)
}

fn _check_file(path: &Path) -> Option<Violation> {
    if path::is_mod_or_lib(path) {
        return None;
    }
    if _has_sibling_dir(path) {
        Some(Violation {
            path: path::rel(path),
            module: _stem(path),
        })
    } else {
        None
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
