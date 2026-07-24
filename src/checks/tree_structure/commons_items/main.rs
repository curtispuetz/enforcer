use std::path::Path;

use crate::{
    c::{files, path, scan},
    t::{ItemsViolation, Outcome},
};

use super::{home, report};

pub fn run() -> bool {
    let (passed, violations) = scan::src_files(_check_file);
    report::print(passed, violations)
}

fn _check_file(path: &Path) -> Outcome<ItemsViolation> {
    let items = _misplaced(path);
    if items.is_empty() {
        Outcome::Passed
    } else {
        Outcome::Failed(ItemsViolation {
            path: path::rel(path),
            items,
        })
    }
}

fn _misplaced(path: &Path) -> Vec<String> {
    let mut ret = Vec::new();
    for item in files::ast_parse(path).items {
        if let Some((commons, desc)) = home::of(&item)
            && !path::in_commons(path, commons)
        {
            ret.push(format!("{desc} -> {commons}"));
        }
    }
    ret
}
