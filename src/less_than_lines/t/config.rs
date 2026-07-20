use std::collections::HashSet;

use crate::s::FILE_CONFIG;

pub struct Config {
    pub max_lines: usize,
    pub ignore: HashSet<String>,
}

impl Config {
    pub fn new() -> Self {
        let mut ignore = HashSet::new();
        for path in FILE_CONFIG.less_than_lines.ignore.iter() {
            ignore.insert(path.replace('\\', "/"));
        }
        Config {
            max_lines: FILE_CONFIG.less_than_lines.num,
            ignore,
        }
    }
}
