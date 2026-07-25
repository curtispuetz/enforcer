use serde::Deserialize;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Comments {
    #[serde(default = "_default_trailing_max")]
    pub max_trailing_comment_len: usize,
}

impl Default for Comments {
    fn default() -> Self {
        Comments {
            max_trailing_comment_len: _default_trailing_max(),
        }
    }
}

fn _default_trailing_max() -> usize {
    20
}
