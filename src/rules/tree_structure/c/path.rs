use super::util;
use crate::{rules::c::path, s::ROOT};
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

pub fn absolute(segments: &[String], file: &Path) -> Option<Vec<String>> {
    let mut acc: Vec<String> = Vec::new();
    for (i, seg) in segments.iter().enumerate() {
        match seg.as_str() {
            "crate" if i == 0 => acc = vec!["crate".to_string()],
            "self" if i == 0 => acc = module(file)?,
            "super" => {
                if i == 0 {
                    acc = module(file)?;
                }
                acc.pop()?;
            }
            _ => acc.push(seg.clone()),
        }
    }
    (acc.first().map(String::as_str) == Some("crate")).then_some(acc)
}

pub fn under_dir(path: &Path, dir: &str) -> bool {
    let rel = path.strip_prefix(ROOT.as_path()).unwrap_or(path);
    rel.components()
        .any(|c| matches!(c, Component::Normal(s) if s.to_str() == Some(dir)))
}

pub fn in_common(path: &Path, name: &str) -> bool {
    under_dir(path, name) || util::common_file_kind(path) == Some(name)
}
