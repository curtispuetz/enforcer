use std::{collections::HashMap, path::Path};

use crate::{
    c::{files, path, scan},
    t::{ItemsViolation, Outcome},
};

use super::{free_calls, impls, report, t::TypeDef, type_defs};

pub fn run() -> bool {
    let type_defs = type_defs::find();
    let (passed, violations) = scan::src_files(|path| _check_file(path, &type_defs));
    report::print(passed, violations)
}

fn _check_file(
    path: &Path,
    type_defs: &HashMap<String, Vec<TypeDef>>,
) -> Outcome<ItemsViolation> {
    let file = files::ast_parse(path);
    let mut items = impls::misplaced(&file, path, type_defs);
    items.extend(free_calls::foreign(&file, path));
    if items.is_empty() {
        Outcome::Passed
    } else {
        Outcome::Failed(ItemsViolation {
            path: path::rel(path),
            items,
        })
    }
}
