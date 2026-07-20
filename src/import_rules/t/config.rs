use std::collections::HashSet;

use crate::{
    import_rules::macros,
    s::{EXISTING_SRC_DIRS, FILE_CONFIG},
};

#[derive(Default)]
pub struct Config {
    pub ignore_exported_macros: bool,
    pub exported_macros: HashSet<String>,
}

impl Config {
    pub fn new() -> Self {
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
}
