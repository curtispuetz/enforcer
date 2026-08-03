use {
    super::{
        count, report,
        t::{Config, Violation},
    },
    crate::{
        rules::c::{path, scan},
        t::Outcome,
    },
    std::path::Path,
};

pub fn run() -> bool {
    scan::run_with_config(Config::new(), _check_file, report::print)
}

fn _check_file(path: &Path, config: &Config) -> Outcome<Violation> {
    if !path::is_mod_or_lib(path) {
        return Outcome::Skipped;
    }
    let module = path::rel(path.parent().unwrap_or(path));
    if config.ignore.contains(&module) {
        return Outcome::Skipped;
    }
    let count = count::mods(path);
    if count > config.max {
        Outcome::Failed(Violation { module, count })
    } else {
        Outcome::Passed
    }
}
