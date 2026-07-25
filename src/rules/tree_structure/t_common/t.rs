use std::{collections::HashMap, path::PathBuf};

pub struct TypeDef {
    pub path: PathBuf,
    pub module: Vec<String>,
    pub is_public: bool,
}

pub struct Defs {
    pub types: HashMap<String, Vec<TypeDef>>,
    pub traits: HashMap<String, Vec<TypeDef>>,
}
