use std::path::Path;

use crate::{checks::t::Results, s::EXISTING_SRC_DIRS, t::Outcome};

use super::files;

pub fn run<V>(
    check: impl FnMut(&Path) -> Outcome<V>,
    print: impl FnOnce(Results<V>) -> bool,
) -> bool {
    let r = src_files(check);
    print(r)
}

pub fn run_with_config<V, C>(
    config: C,
    mut check: impl FnMut(&Path, &C) -> Outcome<V>,
    print: impl FnOnce(C, Results<V>) -> bool,
) -> bool {
    let r = src_files(|path| check(path, &config));
    print(config, r)
}

pub fn src_files<V>(mut check: impl FnMut(&Path) -> Outcome<V>) -> Results<V> {
    let mut r = Results::new();
    for dir_name in EXISTING_SRC_DIRS.iter() {
        for path in files::rs(dir_name) {
            match check(&path) {
                Outcome::Skipped => {}
                Outcome::Passed => r.passed += 1,
                Outcome::Failed(violation) => r.violations.push(violation),
            }
        }
    }
    r
}
