use std::{
    fs,
    path::{Path, PathBuf},
};

use super::util;
use crate::{
    rules::s::COMMON,
    s::{EXISTING_SRC_DIRS, ROOT},
};

pub fn common() -> Vec<PathBuf> {
    let mut dirs = Vec::new();
    for dir_name in EXISTING_SRC_DIRS.iter() {
        _collect(&ROOT.join(dir_name), &mut dirs);
    }
    dirs
}

pub fn common_kind(path: &Path) -> Option<&'static str> {
    if path.is_dir() {
        let name = path.file_name()?.to_str()?;
        return COMMON.into_iter().find(|c| *c == name);
    }
    util::common_file_kind(path)
}

pub fn ancestor(module: &Path) -> Option<(PathBuf, &'static str)> {
    let mut parent = module.parent();
    while let Some(p) = parent {
        if p == ROOT.as_path() {
            break;
        }
        if let Some(kind) = common_kind(p) {
            return Some((p.to_path_buf(), kind));
        }
        parent = p.parent();
    }
    None
}

pub fn root_file(module: &Path) -> PathBuf {
    if module.is_dir() {
        return module.join("mod.rs");
    }
    module.to_path_buf()
}

fn _collect(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if common_kind(&path).is_some() {
            out.push(path.clone());
        }
        if path.is_dir() {
            _collect(&path, out);
        }
    }
}
