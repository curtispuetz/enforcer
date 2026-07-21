use std::path::Path;

use crate::{c::files, s::EXISTING_SRC_DIRS};

pub fn src_files<V>(mut check: impl FnMut(&Path) -> Option<V>) -> (usize, Vec<V>) {
    let mut passed = 0;
    let mut violations = Vec::new();
    for dir_name in EXISTING_SRC_DIRS.iter() {
        for path in files::rs(dir_name) {
            match check(&path) {
                Some(violation) => violations.push(violation),
                None => passed += 1,
            }
        }
    }
    (passed, violations)
}
