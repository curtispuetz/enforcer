use std::path::Path;

use crate::c::{path, scan};

use super::{classify, issues, report, t::Violation};

pub fn run() -> bool {
    let (passed, violations) = scan::src_files(_check_file);
    report::print(passed, violations)
}

fn _check_file(path: &Path) -> Option<Violation> {
    let kind = classify::kind_of(path)?;
    let items = issues::of(path, kind);
    if items.is_empty() {
        None
    } else {
        Some(Violation {
            path: path::rel(path),
            items,
        })
    }
}
