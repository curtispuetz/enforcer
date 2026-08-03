use {
    super::path,
    crate::s::{EXISTING_SRC_DIRS, ROOT},
    std::{
        fs,
        path::{Path, PathBuf},
    },
};

pub fn all() -> Vec<PathBuf> {
    let mut paths = Vec::new();
    for dir_name in EXISTING_SRC_DIRS.iter() {
        paths.extend(rs(dir_name));
    }
    paths
}

pub fn rs(dir_name: &str) -> Vec<PathBuf> {
    let dir = ROOT.join(dir_name);
    let pattern = format!("{}/**/*.rs", dir.to_string_lossy().replace('\\', "/"));
    glob::glob(&pattern)
        .expect("invalid glob")
        .filter_map(|p| p.ok())
        .filter(|p| !path::ignored(p))
        .collect()
}

pub fn ast_parse(path: &Path) -> syn::File {
    let source = fs::read_to_string(path).expect("failed to read file");
    syn::parse_file(&source).expect("failed to parse file")
}
