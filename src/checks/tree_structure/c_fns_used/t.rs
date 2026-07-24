use std::path::PathBuf;

pub type CfKey = (Vec<String>, String);

pub struct CFn {
    pub name: String,
    pub module: Vec<String>,
    pub parent: Vec<String>,
    pub path: PathBuf,
}
