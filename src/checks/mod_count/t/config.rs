use std::collections::HashSet;

use crate::s::FILE_CONFIG;

pub struct Config {
    pub max: usize,
    pub ignore: HashSet<String>,
}

impl Config {
    pub fn new() -> Self {
        let mut ignore = HashSet::new();
        for path in FILE_CONFIG.mod_count.ignore.iter() {
            ignore.insert(path.replace('\\', "/"));
        }
        Config {
            max: FILE_CONFIG.mod_count.max,
            ignore,
        }
    }
}
