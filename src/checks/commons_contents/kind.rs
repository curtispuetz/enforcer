use std::path::Path;

use crate::c::path;

static COMMONS: [&str; 5] = ["c", "cnst", "ext_traits", "s", "t"];

pub fn of(path: &Path) -> Option<&'static str> {
    _stem(path).or_else(|| _dir(path))
}

fn _stem(path: &Path) -> Option<&'static str> {
    _named(path.file_stem()?.to_str()?)
}

fn _dir(path: &Path) -> Option<&'static str> {
    path::file_dir(path)?
        .iter()
        .rev()
        .find_map(|segment| _named(segment))
}

fn _named(name: &str) -> Option<&'static str> {
    COMMONS.into_iter().find(|commons| *commons == name)
}
