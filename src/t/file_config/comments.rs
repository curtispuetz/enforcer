use serde::Deserialize;

#[derive(Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Comments {
    pub max_trailing_comment_len: usize,
}

impl Default for Comments {
    fn default() -> Self {
        Comments {
            max_trailing_comment_len: 20,
        }
    }
}
