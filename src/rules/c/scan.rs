use {
    super::files,
    crate::{rules::t::Results, t::Outcome},
    std::path::Path,
};

pub fn run<V>(
    rule: impl FnMut(&Path) -> Outcome<V>,
    print: impl FnOnce(Results<V>) -> bool,
) -> bool {
    let r = src_files(rule);
    print(r)
}

pub fn run_with_config<V, C>(
    config: C,
    mut rule: impl FnMut(&Path, &C) -> Outcome<V>,
    print: impl FnOnce(C, Results<V>) -> bool,
) -> bool {
    let r = src_files(|path| rule(path, &config));
    print(config, r)
}

pub fn src_files<V>(mut rule: impl FnMut(&Path) -> Outcome<V>) -> Results<V> {
    let mut r = Results::new();
    for path in files::all() {
        match rule(&path) {
            Outcome::Skipped => {}
            Outcome::Passed => r.passed += 1,
            Outcome::Failed(violation) => r.violations.push(violation),
        }
    }
    r
}
