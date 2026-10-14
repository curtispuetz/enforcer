use {
    super::{files, graph},
    crate::{
        rules::t::{FileImports, Results},
        s::EXISTING_SRC_DIRS,
        t::Outcome,
    },
    std::{cell::LazyCell, path::Path},
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

pub fn run_with_graph<V, C>(
    config: C,
    mut rule: impl FnMut(&Path, &C, &LazyCell<Vec<FileImports>>) -> Outcome<V>,
    print: impl FnOnce(C, Results<V>) -> bool,
) -> bool {
    let index: LazyCell<Vec<FileImports>> = LazyCell::new(graph::build);
    run_with_config(config, |path, config| rule(path, config, &index), print)
}

// not-obvious: src/ and tests/ build as separate crates, so code in one can't be
// shared with the other; each dir is collected and grouped on its own.
pub fn per_src_dir<T, G>(
    collect: impl Fn(&str) -> Vec<T>,
    group: impl Fn(Vec<T>) -> Vec<G>,
) -> (usize, Vec<G>) {
    let per_dir: Vec<(usize, Vec<G>)> = EXISTING_SRC_DIRS
        .iter()
        .map(|dir_name| {
            let items = collect(dir_name);
            (items.len(), group(items))
        })
        .collect();
    let total = per_dir.iter().map(|(count, _)| count).sum();
    (total, per_dir.into_iter().flat_map(|(_, groups)| groups).collect())
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
