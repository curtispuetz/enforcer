use std::collections::HashSet;

use colored::Colorize;

use crate::{
    rules::{
        c::scan,
        tree_structure::t::{FileViolation, PartReport},
    },
    s::{EXISTING_SRC_DIRS, FILE_CONFIG},
};

use super::{
    verify, macros,
    t::{Config, Violation},
};

pub fn part() -> PartReport {
    let config = _init_config();
    let res = scan::src_files(|path| verify::file::run(path, &config));
    PartReport {
        name: "import-rules",
        unit: "files",
        passed: res.passed,
        violations: res.violations.into_iter().map(_render).collect(),
    }
}

fn _render(violation: Violation) -> FileViolation {
    let mut lines = Vec::new();
    for import in &violation.imports {
        lines.push(import.text.red().to_string());
        lines.push(format!("  {}", import.reason.dimmed()));
    }
    FileViolation {
        path: violation.path,
        lines,
    }
}

fn _init_config() -> Config {
    let ignore_exported_macros =
        FILE_CONFIG.tree_structure.import_rules.ignore_export_macros;
    let mut exported_macros = HashSet::new();
    if ignore_exported_macros {
        for dir_name in EXISTING_SRC_DIRS.iter() {
            exported_macros.extend(macros::collect_exported(dir_name));
        }
    }
    Config {
        ignore_exported_macros,
        exported_macros,
    }
}
