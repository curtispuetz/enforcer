use std::collections::HashMap;

#[derive(Clone)]
pub enum Tree {
    Path(String, Box<Tree>),
    Name(String),
    Rename(String, String),
    Glob,
    Group(Vec<Tree>),
}

pub struct Moved {
    pub path: Vec<String>,
    pub leaf: Tree,
}

pub struct Removal<'a> {
    pub rewrites: &'a HashMap<Vec<String>, Vec<String>>,
    pub moved: Vec<Moved>,
}
