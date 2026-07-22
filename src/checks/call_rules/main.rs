use std::path::Path;

use crate::c::{files, path, scan};

use super::{find, report, t::Violation};

pub fn run() -> bool {
    let (passed, violations) = scan::src_files(_check_file);
    report(passed, violations)
}

fn _check_file(path: &Path) -> Option<Violation> {
    let items = find::violations(&files::parse(path));
    if items.is_empty() {
        None
    } else {
        Some(Violation {
            path: path::rel(path),
            items,
        })
    }
}
