use std::fs;
use std::path::{Path, PathBuf};

pub fn rs(root: &Path, dir_name: &str) -> Vec<PathBuf> {
    let dir = root.join(dir_name);
    let pattern = format!("{}/**/*.rs", dir.to_string_lossy().replace('\\', "/"));
    glob::glob(&pattern)
        .expect("invalid glob")
        .filter_map(|p| p.ok())
        .collect()
}

pub fn parse(path: &Path) -> syn::File {
    let source = fs::read_to_string(path).expect("failed to read file");
    syn::parse_file(&source).expect("failed to parse file")
}
