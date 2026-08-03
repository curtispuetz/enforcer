use {crate::s::FILE_CONFIG, std::collections::HashSet};

pub struct Config {
    pub max_lines: usize,
    pub ignore: HashSet<String>,
}

impl Config {
    pub fn new() -> Self {
        let mut ignore = HashSet::new();
        for path in FILE_CONFIG.file_sizes.ignore.iter() {
            ignore.insert(path.replace('\\', "/"));
        }
        Config {
            max_lines: FILE_CONFIG.file_sizes.max,
            ignore,
        }
    }
}
