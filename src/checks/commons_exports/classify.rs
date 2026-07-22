use std::path::Path;

use crate::c::path;

pub fn is_commons_root(path: &Path) -> bool {
    if path.file_name().and_then(|n| n.to_str()) != Some("mod.rs") {
        return false;
    }
    matches!(path::parent_name(path), Some("t" | "s"))
}
