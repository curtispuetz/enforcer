use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::{
    checks::{c::path, s::COMMON, t::Results, tree_structure::c::path as path2},
    s::{EXISTING_SRC_DIRS, ROOT},
    t::ItemsViolation,
};

pub fn check() -> Results<ItemsViolation> {
    let mut r = Results::new();
    for module in _common() {
        match _bad_ancestor(&module) {
            Some(ancestor) => r.violations.push(ItemsViolation {
                path: path::rel(&module),
                items: vec![format!(
                    "nested inside common module `{}`",
                    path::rel(&ancestor)
                )],
            }),
            None => r.passed += 1,
        }
    }
    r
}

fn _common() -> Vec<PathBuf> {
    let mut dirs = Vec::new();
    for dir_name in EXISTING_SRC_DIRS.iter() {
        _collect(&ROOT.join(dir_name), &mut dirs);
    }
    dirs
}

fn _collect(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if _common_kind(&path).is_some() {
            out.push(path.clone());
        }
        if path.is_dir() {
            _collect(&path, out);
        }
    }
}

fn _bad_ancestor(module: &Path) -> Option<PathBuf> {
    let kind = _common_kind(module)?;
    let (ancestor, ancestor_kind) = _nearest_common_ancestor(module)?;
    // not-obvious: c-in-c is the only nesting allowed, so it's the one pairing we skip.
    if kind == "c" && ancestor_kind == "c" {
        return None;
    }
    Some(ancestor)
}

fn _nearest_common_ancestor(module: &Path) -> Option<(PathBuf, &'static str)> {
    let mut parent = module.parent();
    while let Some(p) = parent {
        if p == ROOT.as_path() {
            break;
        }
        if let Some(kind) = _common_kind(p) {
            return Some((p.to_path_buf(), kind));
        }
        parent = p.parent();
    }
    None
}

fn _common_kind(path: &Path) -> Option<&'static str> {
    if path.is_dir() {
        let name = path.file_name()?.to_str()?;
        COMMON.into_iter().find(|c| *c == name)
    } else {
        path2::common_file_kind(path)
    }
}
