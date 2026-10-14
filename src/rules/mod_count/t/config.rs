use {
    crate::{rules::c::path, s::FILE_CONFIG},
    std::collections::HashSet,
};

const MERGE_MARGIN: usize = 5;

pub struct Config {
    pub max: usize,
    pub ignore: HashSet<String>,
    pub merge_max_lines: usize,
}

impl Config {
    pub fn new() -> Self {
        Config {
            max: FILE_CONFIG.mod_count.max,
            ignore: path::ignore_set(&FILE_CONFIG.mod_count.ignore),
            merge_max_lines: FILE_CONFIG.file_sizes.max.saturating_sub(MERGE_MARGIN),
        }
    }
}
