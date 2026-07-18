use std::path::{Path, PathBuf};

pub fn rs_files(root: &Path, dir_name: &str) -> Vec<PathBuf> {
    let dir = root.join(dir_name);
    let pattern = format!("{}/**/*.rs", dir.to_string_lossy().replace('\\', "/"));
    glob::glob(&pattern)
        .expect("invalid glob")
        .filter_map(|p| p.ok())
        .collect()
}
