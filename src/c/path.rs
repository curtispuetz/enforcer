use crate::s::ROOT;
use std::path::Path;

pub fn rel(path: &Path) -> String {
    path.strip_prefix(ROOT.as_path())
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}
