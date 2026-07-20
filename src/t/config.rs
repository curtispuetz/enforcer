use std::collections::HashSet;

use crate::{macros, s::main::EXISTING_SRC_DIRS};

#[derive(Default)]
pub struct Config {
    pub ignore_exported_macros: bool,
    pub exported_macros: HashSet<String>,
}

impl Config {
    pub fn new() -> Self {
        let ignore_exported_macros =
            std::env::args().any(|a| a == "--ignore-exported-macros");
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
