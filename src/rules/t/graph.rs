use std::{
    collections::{BTreeSet, HashMap, HashSet},
    path::PathBuf,
};

pub struct FileImports {
    pub path: PathBuf,
    pub dir: Vec<String>,
    pub module: Vec<String>,
    pub targets: Vec<Vec<String>>,
    pub resolved: Vec<Vec<String>>,
}

#[derive(Default)]
pub struct ModuleExports {
    pub children: HashSet<String>,
    pub items: HashSet<String>,
    pub named: HashMap<String, Vec<String>>,
    pub globs: Vec<Vec<String>>,
}

pub type Binding = (String, Vec<String>);

pub type ExportTables = HashMap<Vec<String>, ModuleExports>;

pub struct ChildLinks {
    pub name: String,
    pub is_common: bool,
    pub imports: BTreeSet<String>,
    pub importers: BTreeSet<Importer>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum Importer {
    Sibling(String),
    Parent,
    Elsewhere(PathBuf),
}
