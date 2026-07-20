use std::{path::PathBuf, sync::LazyLock};

pub static EXISTING_SRC_DIRS: LazyLock<Vec<&'static str>> = LazyLock::new(|| {
    ["src", "tests"]
        .into_iter()
        .filter(|dir_name| ROOT.join(dir_name).exists())
        .collect()
});

pub static ROOT: LazyLock<PathBuf> = LazyLock::new(|| {
    _manifest_dir().expect("Failed to find Cargo.toml in any parent directory")
});

fn _manifest_dir() -> Option<PathBuf> {
    let mut dir = std::env::current_dir().ok()?;
    loop {
        if dir.join("Cargo.toml").exists() {
            return Some(dir);
        }
        dir = dir.parent()?.to_path_buf();
    }
}
