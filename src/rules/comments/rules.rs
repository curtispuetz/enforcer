use super::{
    classify,
    cnst::{NOT_OBVIOUS, TODO},
    t::{BadComment, Comment, Config},
};

pub fn eval(comment: &Comment, config: &Config) -> Option<BadComment> {
    if _is_allowed_prefix(comment) || _allowed_trailing(comment, config) {
        return None;
    }
    Some(BadComment {
        line: comment.line,
        text: comment.full.clone(),
        reason: _reason(comment, config),
    })
}

fn _is_allowed_prefix(comment: &Comment) -> bool {
    classify::not_obvious(comment) || classify::todo(comment)
}

fn _allowed_trailing(comment: &Comment, config: &Config) -> bool {
    !comment.is_block
        && comment.trailing
        && _trailing_len(&comment.full) <= config.max_trailing_len
}

fn _trailing_len(full: &str) -> usize {
    let slashes = full.chars().take_while(|c| *c == '/').count();
    let body = full[slashes..].trim_start();
    slashes + body.chars().count()
}

fn _reason(comment: &Comment, config: &Config) -> String {
    if !comment.is_block && comment.trailing {
        format!(
            "trailing comment is {} characters, over the max of {}",
            _trailing_len(&comment.full),
            config.max_trailing_len
        )
    } else {
        format!("comment must start with '{NOT_OBVIOUS}' or '{TODO}'")
    }
}
