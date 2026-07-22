use crate::s::ROOT;
use std::path::{Component, Path};

pub fn rel(path: &Path) -> String {
    path.strip_prefix(ROOT.as_path())
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

pub fn under_dir(path: &Path, dir: &str) -> bool {
    let rel = path.strip_prefix(ROOT.as_path()).unwrap_or(path);
    rel.components()
        .any(|c| matches!(c, Component::Normal(s) if s.to_str() == Some(dir)))
}

pub fn parent_name(path: &Path) -> Option<&str> {
    path.parent()?.file_name()?.to_str()
}

pub fn is_mod_or_lib(path: &Path) -> bool {
    matches!(
        path.file_name().and_then(|n| n.to_str()),
        Some("mod.rs" | "lib.rs")
    )
}
