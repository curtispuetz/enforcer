use {
    super::{find, report},
    crate::{
        rules::c::{files, outcome, scan},
        t::{ItemsViolation, Outcome},
    },
    std::path::Path,
};

pub fn run() -> bool {
    scan::run(_check_file, report::print)
}

fn _check_file(path: &Path) -> Outcome<ItemsViolation> {
    outcome::of_items(path, find::violations(&files::ast_parse(path)))
}
