use std::{collections::HashMap, path::Path};

use crate::{
    checks::{
        c::{files, path, scan},
        tree_structure::t::{FileViolation, PartReport},
    },
    t::{ItemsViolation, Outcome},
};

use super::{impls, t::TypeDef, type_defs};

pub fn part() -> PartReport {
    let type_defs = type_defs::find();
    let res = scan::src_files(|path| _check_file(path, &type_defs));
    PartReport {
        name: "t-common",
        unit: "files",
        passed: res.passed,
        violations: res.violations.into_iter().map(FileViolation::new).collect(),
    }
}

fn _check_file(
    path: &Path,
    type_defs: &HashMap<String, Vec<TypeDef>>,
) -> Outcome<ItemsViolation> {
    let file = files::ast_parse(path);
    let items = impls::misplaced(&file, path, type_defs);
    if items.is_empty() {
        Outcome::Passed
    } else {
        Outcome::Failed(ItemsViolation {
            path: path::rel(path),
            items,
        })
    }
}
