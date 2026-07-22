use std::path::Path;

use super::t::Kind;

pub fn kind_of(path: &Path) -> Option<Kind> {
    if path.file_name().and_then(|n| n.to_str()) != Some("mod.rs") {
        return None;
    }
    match _parent_name(path) {
        Some("t") => Some(Kind::Types),
        Some("s") => Some(Kind::Statics),
        _ => None,
    }
}

fn _parent_name(path: &Path) -> Option<&str> {
    path.parent()?.file_name()?.to_str()
}
