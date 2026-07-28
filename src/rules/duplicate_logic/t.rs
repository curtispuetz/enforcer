use std::{collections::HashSet, fmt};

use syn::Block;

use crate::s::FILE_CONFIG;

pub struct Config {
    pub min_stmts: usize,
    pub max_holes: usize,
    pub min_nodes_per_hole: usize,
    pub ignore: HashSet<String>,
}

impl Config {
    pub fn new() -> Self {
        let cfg = &FILE_CONFIG.duplicate_logic;
        let ignore = cfg.ignore.iter().cloned().collect();
        Config {
            min_stmts: cfg.min_stmts,
            max_holes: cfg.max_holes,
            min_nodes_per_hole: cfg.min_nodes_per_hole,
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

#[derive(Clone)]
pub enum HoleKind {
    Field,
    Method,
    Literal,
    Type,
}

impl fmt::Display for HoleKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match self {
            HoleKind::Field => "field",
            HoleKind::Method => "method",
            HoleKind::Literal => "literal",
            HoleKind::Type => "type",
        };
        write!(f, "{name}")
    }
}

#[derive(Clone)]
pub struct Leaf {
    pub kind: HoleKind,
    pub text: String,
}

pub struct Shape {
    pub block: Block,
    pub leaves: Vec<Leaf>,
    pub nodes: usize,
}

pub struct Hole {
    pub kind: HoleKind,
    pub values: Vec<String>,
}

pub struct Candidate {
    pub canonical: Block,
    pub shape: Shape,
    pub occurrence: Occurrence,
}

pub struct Group {
    pub id: String,
    pub holes: Vec<Hole>,
    pub occurrences: Vec<Occurrence>,
}
