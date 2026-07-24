use crate::{
    checks::{c::path, s::COMMON},
    s::ROOT,
};
use std::path::{Component, Path};

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

pub fn module(path: &Path) -> Option<Vec<String>> {
    let mut segments = file_dir(path)?;
    if !path::is_mod_or_lib(path) {
        segments.push(path.file_stem()?.to_str()?.to_string());
    }
    Some(segments)
}

pub fn common_file_kind(path: &Path) -> Option<&'static str> {
    if path.extension().and_then(|e| e.to_str()) != Some("rs") {
        return None;
    }
    let stem = path.file_stem()?.to_str()?;
    COMMON.into_iter().find(|c| *c == stem)
}

pub fn under_dir(path: &Path, dir: &str) -> bool {
    let rel = path.strip_prefix(ROOT.as_path()).unwrap_or(path);
    rel.components()
        .any(|c| matches!(c, Component::Normal(s) if s.to_str() == Some(dir)))
}

pub fn in_common(path: &Path, name: &str) -> bool {
    under_dir(path, name) || common_file_kind(path) == Some(name)
}
