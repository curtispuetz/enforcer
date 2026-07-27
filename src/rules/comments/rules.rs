use super::t::{BadComment, Comment, Config};

const NOT_OBVIOUS: &str = "not-obvious: ";
const TODO: &str = "TODO";

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
    let body = _inner(&comment.full, comment.is_block);
    let body = body.trim_start();
    body.starts_with(NOT_OBVIOUS) || body.starts_with(TODO)
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

fn _inner(full: &str, is_block: bool) -> &str {
    if is_block {
        let body = full.strip_prefix("/*").unwrap_or(full);
        let body = body.strip_suffix("*/").unwrap_or(body);
        body.trim_start_matches(['*', '!'])
    } else {
        let body = full.trim_start_matches('/');
        body.strip_prefix('!').unwrap_or(body)
    }
}
