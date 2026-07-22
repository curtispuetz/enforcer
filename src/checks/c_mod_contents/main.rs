use std::path::Path;

use crate::{
    c::{files, path, scan},
    checks::t::ItemsViolation,
};

use super::{check, report};

pub fn run() -> bool {
    let (passed, violations) = scan::src_files(_check_file);
    report::print(passed, violations)
}

fn _check_file(path: &Path) -> Option<ItemsViolation> {
    if !_is_c_mod(path) {
        return None;
    }
    let file = files::parse(path);
    let items = if _has_sub_c(path) {
        check::inception::violations(path, &file)
    } else {
        check::simple::violations(&file)
    };
    if items.is_empty() {
        None
    } else {
        Some(ItemsViolation {
            path: path::rel(path),
            items,
        })
    }
}

fn _is_c_mod(path: &Path) -> bool {
    path.file_name().and_then(|n| n.to_str()) == Some("mod.rs")
        && path::parent_name(path) == Some("c")
}

fn _has_sub_c(path: &Path) -> bool {
    path.parent().map(|p| p.join("c").is_dir()).unwrap_or(false)
}
