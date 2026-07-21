use std::collections::HashSet;

use crate::{
    c::scan,
    s::{EXISTING_SRC_DIRS, FILE_CONFIG},
};

use super::{check, macros, report::report, t::config::Config};

pub fn run() -> bool {
    let config = _init_config();
    let (passed, violations) = scan::src_files(|path| check::file::run(path, &config));
    report(passed, violations)
}

fn _init_config() -> Config {
    let ignore_exported_macros = FILE_CONFIG.import_rules.ignore_export_macros;
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
