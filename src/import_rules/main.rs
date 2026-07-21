use crate::{
    c::files,
    import_rules::{
        check,
        report::report,
        t::{config::Config, violation::Violation},
    },
    s::{EXISTING_SRC_DIRS, ROOT},
};

pub fn run() -> bool {
    let config = Config::new();
    let (passed, violations) = _check_all(&config);
    report(passed, violations)
}

fn _check_all(config: &Config) -> (usize, Vec<Violation>) {
    let mut passed = 0;
    let mut violations = Vec::new();
    for dir_name in EXISTING_SRC_DIRS.iter() {
        let dir = ROOT.join(dir_name);
        for path in files::rs(dir_name) {
            match check::file::run(&path, &dir, config) {
                Some(violation) => violations.push(violation),
                None => passed += 1,
            }
        }
    }
    (passed, violations)
}
