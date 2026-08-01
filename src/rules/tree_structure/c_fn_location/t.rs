use std::path::PathBuf;

pub type CfKey = (Vec<String>, String);

pub struct CFn {
    pub name: String,
    pub module: Vec<String>,
    pub parent: Vec<String>,
    pub path: PathBuf,
}

pub struct Reach {
    pub key: CfKey,
    pub path: PathBuf,
}

pub struct Defs {
    pub cfns: Vec<CFn>,
    pub reaches: Vec<Reach>,
}
