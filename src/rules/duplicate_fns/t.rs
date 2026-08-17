use {
    crate::{rules::c::path, s::FILE_CONFIG},
    std::collections::HashSet,
    syn::ItemFn,
};

pub struct Config {
    pub ignore: HashSet<String>,
}

impl Config {
    pub fn new() -> Self {
        Config {
            ignore: path::ignore_set(&FILE_CONFIG.duplicate_fns.ignore),
        }
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
