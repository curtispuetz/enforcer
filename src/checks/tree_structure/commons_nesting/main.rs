use std::path::{Path, PathBuf};

use crate::{c::path, s::ROOT};

use super::{dirs, report, t::Violation};

pub fn run() -> bool {
    let mut passed = 0;
    let mut violations = Vec::new();
    for module in dirs::commons() {
        match _bad_ancestor(&module) {
            Some(ancestor) => violations.push(Violation {
                path: path::rel(&module),
                ancestor: path::rel(&ancestor),
            }),
            None => passed += 1,
        }
    }
    report::print(passed, violations)
}

fn _bad_ancestor(module: &Path) -> Option<PathBuf> {
    let kind = path::commons_kind(module)?;
    let (ancestor, ancestor_kind) = _nearest_commons_ancestor(module)?;
    // not-obvious: c-in-c is the only nesting allowed, so it's the one pairing we skip.
    if kind == "c" && ancestor_kind == "c" {
        return None;
    }
    Some(ancestor)
}

fn _nearest_commons_ancestor(module: &Path) -> Option<(PathBuf, &'static str)> {
    let mut parent = module.parent();
    while let Some(p) = parent {
        if p == ROOT.as_path() {
            break;
        }
        if let Some(kind) = path::commons_kind(p) {
            return Some((p.to_path_buf(), kind));
        }
        parent = p.parent();
    }
    None
}
