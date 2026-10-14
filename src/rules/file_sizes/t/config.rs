use {
    crate::{rules::c::path, s::FILE_CONFIG},
    std::collections::HashSet,
};

pub struct Config {
    pub max_lines: usize,
    pub ignore: HashSet<String>,
    pub mod_max: usize,
    pub mod_ignore: HashSet<String>,
}

impl Config {
    pub fn new() -> Self {
        Config {
            max_lines: FILE_CONFIG.file_sizes.max,
            ignore: path::ignore_set(&FILE_CONFIG.file_sizes.ignore),
            mod_max: FILE_CONFIG.mod_count.max,
            mod_ignore: path::ignore_set(&FILE_CONFIG.mod_count.ignore),
        }
    }
}
