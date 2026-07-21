use std::path::{Component, Path};

use crate::s::ROOT;

static COMMONS_DIRS: [&str; 4] = ["c", "s", "t", "ext_traits"];

pub fn file_dir_segments(path: &Path) -> Option<Vec<String>> {
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

pub fn is_commons(seg: &str) -> bool {
    COMMONS_DIRS.contains(&seg)
}
