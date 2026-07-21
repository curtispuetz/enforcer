use crate::{
    c::scan,
    import_rules::{check, report::report, t::config::Config},
};

pub fn run() -> bool {
    let config = Config::new();
    let (passed, violations) = scan::src_files(|path| check::file::run(path, &config));
    report(passed, violations)
}
