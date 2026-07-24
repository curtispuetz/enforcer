use std::path::Path;

use crate::{s::EXISTING_SRC_DIRS, t::Outcome};

use super::files;

pub fn src_files<V>(mut check: impl FnMut(&Path) -> Outcome<V>) -> (usize, Vec<V>) {
    let mut passed = 0;
    let mut violations = Vec::new();
    for dir_name in EXISTING_SRC_DIRS.iter() {
        for path in files::rs(dir_name) {
            match check(&path) {
                Outcome::Skipped => {}
                Outcome::Passed => passed += 1,
                Outcome::Failed(violation) => violations.push(violation),
            }
        }
    }
    (passed, violations)
}
