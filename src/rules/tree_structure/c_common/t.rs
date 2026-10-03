use std::path::PathBuf;

pub struct FileImports {
    pub path: PathBuf,
    pub dir: Vec<String>,
    pub module: Vec<String>,
    pub targets: Vec<Vec<String>>,
}

pub struct Folder {
    pub path: PathBuf,
    pub parent: Vec<String>,
    pub segments: Vec<String>,
    pub members: Vec<Member>,
}

pub struct Member {
    pub name: String,
    pub path: PathBuf,
    pub segments: Vec<String>,
}
