use std::collections::HashSet;

use syn::ItemFn;

use crate::s::FILE_CONFIG;

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

pub struct FreeFn {
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
