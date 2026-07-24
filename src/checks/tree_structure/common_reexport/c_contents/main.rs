use std::path::Path;

use crate::{
    c::{files, path, scan},
    t::{ItemsViolation, Outcome},
};

use super::{inception, simple};

pub fn check() -> (usize, Vec<ItemsViolation>) {
    scan::src_files(_check_file)
}

fn _check_file(path: &Path) -> Outcome<ItemsViolation> {
    if !_is_c_mod(path) {
        return Outcome::Skipped;
    }
    let file = files::ast_parse(path);
    let items = if _has_sub_c(path) {
        inception::violations(&file, _needs_allow(path))
    } else {
        simple::violations(&file)
    };
    if items.is_empty() {
        Outcome::Passed
    } else {
        Outcome::Failed(ItemsViolation {
            path: path::rel(path),
            items,
        })
    }
}

fn _is_c_mod(path: &Path) -> bool {
    path.file_name().and_then(|n| n.to_str()) == Some("mod.rs")
        && path::under_dir(path, "c")
}

fn _has_sub_c(path: &Path) -> bool {
    path.parent().map(|p| p.join("c").is_dir()).unwrap_or(false)
}

fn _needs_allow(path: &Path) -> bool {
    path::parent_name(path) == Some("c")
}
