use {
    super::{macros, render, t::Config, verify},
    crate::{
        rules::{c::scan, t::PartReport},
        s::{EXISTING_SRC_DIRS, FILE_CONFIG},
    },
    std::collections::HashSet,
};

pub fn part() -> PartReport {
    let config = _init_config();
    let res = scan::src_files(|path| verify::file::run(path, &config));
    PartReport {
        name: "imports",
        failures_header:
            "The following file(s) have an import pointing up or across the module tree, or written in the wrong form:",
        unit: "files",
        passed: res.passed,
        violations: res.violations.into_iter().map(render::violation).collect(),
    }
}

fn _init_config() -> Config {
    let ignore_exported_macros =
        FILE_CONFIG.tree_structure.imports.ignore_export_macros;
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
