use crate::s::FILE_CONFIG;

pub struct Config {
    pub max_trailing_len: usize,
}

impl Config {
    pub fn new() -> Self {
        Config {
            max_trailing_len: FILE_CONFIG.comments.max_trailing_comment_len,
        }
    }
}
