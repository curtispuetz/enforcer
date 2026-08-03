use {crate::s::FILE_CONFIG, std::collections::HashSet};

pub struct Config {
    pub max: usize,
    pub ignore: HashSet<String>,
}

impl Config {
    pub fn new() -> Self {
        let mut ignore = HashSet::new();
        for entry in FILE_CONFIG.cognitive_complexity.ignore.iter() {
            ignore.insert(entry.replace('\\', "/"));
        }
        Config {
            max: FILE_CONFIG.cognitive_complexity.max,
            ignore,
        }
    }
}
