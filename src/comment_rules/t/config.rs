use crate::s::FILE_CONFIG;

pub struct Config {
    pub max_trailing_len: usize,
}

impl Config {
    pub fn new() -> Self {
        Config {
            max_trailing_len: FILE_CONFIG.comment_rules.max_trailing_comment_len,
        }
    }
}
