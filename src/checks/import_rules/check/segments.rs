use std::path::{Component, Path};

use crate::s::ROOT;

static COMMONS_DIRS: [&str; 4] = ["c", "s", "t", "ext_traits"];

pub fn file_dir(path: &Path) -> Option<Vec<String>> {
    let rel = path.strip_prefix(ROOT.as_path()).ok()?;
    let mut segments = vec!["crate".to_string()];
    // not-obvious: skip(1) drops the top-level src dir (src/ or tests/).
    for component in rel.parent()?.components().skip(1) {
        if let Component::Normal(s) = component {
            segments.push(s.to_str()?.to_string());
        }
    }
    Some(segments)
}

pub fn own_module(path: &Path) -> Option<Vec<String>> {
    let mut segments = file_dir(path)?;
    let stem = path.file_stem()?.to_str()?;
    if !_is_folder_module(stem) {
        segments.push(stem.to_string());
    }
    Some(segments)
}

fn _is_folder_module(stem: &str) -> bool {
    stem == "mod" || stem == "lib"
}

pub fn is_commons(seg: &str) -> bool {
    COMMONS_DIRS.contains(&seg)
}
