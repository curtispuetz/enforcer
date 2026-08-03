use {
    super::{defs, impls, t::Defs},
    crate::{
        rules::{
            c::{files, path},
            tree_structure::{c::parts, t::PartReport},
        },
        t::{ItemsViolation, Outcome},
    },
    std::path::Path,
};

pub fn part() -> PartReport {
    let defs = defs::find();
    parts::from_files("t-common", |path| _check_file(path, &defs))
}

fn _check_file(path: &Path, defs: &Defs) -> Outcome<ItemsViolation> {
    let file = files::ast_parse(path);
    let items = impls::misplaced(&file, path, defs);
    if items.is_empty() {
        Outcome::Passed
    } else {
        Outcome::Failed(ItemsViolation {
            path: path::rel(path),
            items,
        })
    }
}
