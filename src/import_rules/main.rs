use crate::{
    c::files,
    import_rules::{check, report::report, t::config::Config},
    s::main::{EXISTING_SRC_DIRS, ROOT},
};

pub fn run() {
    let config = Config::new();
    let (passed, failed) = _check_all(&config);
    report(passed, failed);
}

fn _check_all(config: &Config) -> (usize, usize) {
    let mut passed = 0;
    let mut failed = 0;
    for dir_name in EXISTING_SRC_DIRS.iter() {
        let (p, f) = _check_dir(dir_name, config);
        passed += p;
        failed += f;
    }
    (passed, failed)
}

fn _check_dir(dir_name: &str, config: &Config) -> (usize, usize) {
    let mut passed = 0;
    let mut failed = 0;
    let dir = ROOT.join(dir_name);
    for path in files::rs(dir_name) {
        if check::file::run(&path, &dir, config) > 0 {
            failed += 1;
        } else {
            passed += 1;
        }
    }
    (passed, failed)
}
