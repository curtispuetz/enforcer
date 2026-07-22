use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::s::{EXISTING_SRC_DIRS, ROOT};

pub fn commons() -> Vec<PathBuf> {
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
        if !path.is_dir() {
            continue;
        }
        if _is_commons(&path) {
            out.push(path.clone());
        }
        _collect(&path, out);
    }
}

fn _is_commons(dir: &Path) -> bool {
    matches!(dir.file_name().and_then(|n| n.to_str()), Some("t" | "s"))
}
