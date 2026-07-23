static COMMONS_DIRS: [&str; 4] = ["c", "s", "t", "ext_traits"];

pub fn is_commons(seg: &str) -> bool {
    COMMONS_DIRS.contains(&seg)
}
