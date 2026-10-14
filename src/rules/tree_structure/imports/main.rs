use {
    super::{autofix, macros, render, t::Config, verify},
    crate::{
        rules::{
            c::{files, scan},
            t::PartReport,
        },
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

pub fn fix() {
    let config = _init_config();
    let mut manual = 0;
    let fixed = files::all()
        .iter()
        .filter_map(|path| {
            let (rewrites, left) = verify::file::fixes(path, &config);
            manual += left;
            (!rewrites.is_empty()).then(|| autofix::file(path, &rewrites))?
        })
        .collect();
    autofix::print(fixed, manual);
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
