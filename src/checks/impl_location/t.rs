use std::path::PathBuf;

pub struct TypeDef {
    pub path: PathBuf,
    pub module: Vec<String>,
    pub is_public: bool,
}
