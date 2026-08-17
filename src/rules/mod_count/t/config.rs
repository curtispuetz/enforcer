use {
    crate::{rules::c::path, s::FILE_CONFIG},
    std::collections::HashSet,
};

pub struct Config {
    pub max: usize,
    pub ignore: HashSet<String>,
}

impl Config {
    pub fn new() -> Self {
        Config {
            max: FILE_CONFIG.mod_count.max,
            ignore: path::ignore_set(&FILE_CONFIG.mod_count.ignore),
        }
    }
}
