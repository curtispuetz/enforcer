use std::collections::HashSet;

use syn::Block;

use crate::s::FILE_CONFIG;

pub struct Config {
    pub min_stmts: usize,
    pub ignore: HashSet<String>,
}

impl Config {
    pub fn new() -> Self {
        let cfg = &FILE_CONFIG.duplicate_logic;
        let ignore = cfg.ignore.iter().cloned().collect();
        Config {
            min_stmts: cfg.min_stmts,
            ignore,
        }
    }
}

#[derive(Clone)]
pub struct Occurrence {
    pub path: String,
    pub start: usize,
    pub end: usize,
}

pub struct Candidate {
    pub canonical: Block,
    pub occurrence: Occurrence,
}

pub struct Group {
    pub id: String,
    pub occurrences: Vec<Occurrence>,
}
