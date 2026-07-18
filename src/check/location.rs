use std::path::{Component, Path};

static EXEMPT_DIRS: [&str; 3] = ["s", "t", "ext_traits"];

pub fn file_dir_segments(path: &Path, src_dir: &Path) -> Option<Vec<String>> {
    let rel = path.strip_prefix(src_dir).ok()?;
    let mut segments = vec!["crate".to_string()];
    for component in rel.parent()?.components() {
        if let Component::Normal(s) = component {
            segments.push(s.to_str()?.to_string());
        }
    }
    Some(segments)
}

// 's' is a static-data directory, 't' is a types directory; both are exempt from the rules.
pub fn is_static_or_types(seg: &str) -> bool {
    EXEMPT_DIRS.contains(&seg)
}
