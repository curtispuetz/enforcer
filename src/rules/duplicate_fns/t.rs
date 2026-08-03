use {crate::s::FILE_CONFIG, std::collections::HashSet, syn::ItemFn};

pub struct Config {
    pub ignore: HashSet<String>,
}

impl Config {
    pub fn new() -> Self {
        let mut ignore = HashSet::new();
        for entry in FILE_CONFIG.duplicate_fns.ignore.iter() {
            ignore.insert(entry.replace('\\', "/"));
        }
        Config { ignore }
    }
}

pub struct CollectedFn {
    pub path: String,
    pub name: String,
    pub line: usize,
    pub item: ItemFn,
}

pub struct Duplicate {
    pub path: String,
    pub name: String,
    pub line: usize,
}

pub struct Group {
    pub members: Vec<Duplicate>,
}
