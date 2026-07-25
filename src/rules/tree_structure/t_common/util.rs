use std::path::Path;

use crate::rules::tree_structure::c::path;

pub fn is_t_common(path: &Path) -> bool {
    path::in_common(path, "t")
}
