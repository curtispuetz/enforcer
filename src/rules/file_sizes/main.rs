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
    let relative = path::rel(path);
    if config.ignore.contains(&relative) {
        return Outcome::Skipped;
    }
    let lines = count::lines(path);
    if lines > config.max_lines {
        Outcome::Failed(Violation {
            path: relative,
            lines,
        })
    } else {
        Outcome::Passed
    }
}
