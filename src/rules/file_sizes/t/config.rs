use {
    crate::{rules::c::path, s::FILE_CONFIG},
    std::collections::HashSet,
};

pub struct Config {
    pub max_lines: usize,
    pub ignore: HashSet<String>,
}

impl Config {
    pub fn new() -> Self {
        Config {
            max_lines: FILE_CONFIG.file_sizes.max,
            ignore: path::ignore_set(&FILE_CONFIG.file_sizes.ignore),
        }
    }
}
