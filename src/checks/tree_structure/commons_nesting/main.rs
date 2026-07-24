use std::path::Path;

use crate::{c::path, s::ROOT};

use super::{dirs, report, t::Violation};

pub fn run() -> bool {
    let mut passed = 0;
    let mut violations = Vec::new();
    for dir in dirs::commons() {
        match _nested_ancestor(&dir) {
            Some(ancestor) => violations.push(Violation {
                path: path::rel(&dir),
                ancestor,
            }),
            None => passed += 1,
        }
    }
    report::print(passed, violations)
}

fn _nested_ancestor(dir: &Path) -> Option<String> {
    let name = dir.file_stem().and_then(|n| n.to_str())?;
    let mut parent = dir.parent();
    while let Some(p) = parent {
        if p == ROOT.as_path() {
            break;
        }
        if p.file_name().and_then(|n| n.to_str()) == Some(name) {
            return Some(path::rel(p));
        }
        parent = p.parent();
    }
    None
}
