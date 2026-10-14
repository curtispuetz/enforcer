use {
    super::{
        count, report, suggest,
        t::{Config, Violation},
    },
    crate::{
        rules::{
            c::{path, scan},
            t::FileImports,
        },
        t::Outcome,
    },
    std::{cell::LazyCell, path::Path},
};

pub fn run() -> bool {
    scan::run_with_graph(Config::new(), _check_file, report::print)
}

fn _check_file(
    path: &Path,
    config: &Config,
    index: &LazyCell<Vec<FileImports>>,
) -> Outcome<Violation> {
    if !path::is_mod_or_lib(path) {
        return Outcome::Skipped;
    }
    let module = path::rel(path.parent().unwrap_or(path));
    if config.ignore.contains(&module) {
        return Outcome::Skipped;
    }
    let count = count::mods(path);
    if count > config.max {
        let fixes = suggest::fixes(path, config, index);
        Outcome::Failed(Violation {
            module,
            count,
            fixes,
        })
    } else {
        Outcome::Passed
    }
}
