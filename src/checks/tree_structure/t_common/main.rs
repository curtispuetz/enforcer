use std::{collections::HashMap, path::Path};

use crate::{
    c::{files, path, scan},
    checks::tree_structure::{c, t::PartReport},
    t::{ItemsViolation, Outcome},
};

use super::{free_calls, impls, t::TypeDef, type_defs};

pub fn part() -> PartReport {
    let type_defs = type_defs::find();
    let (passed, violations) = scan::src_files(|path| _check_file(path, &type_defs));
    PartReport {
        name: "t-common",
        unit: "files",
        passed,
        violations: violations.into_iter().map(c::file_violation).collect(),
    }
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
