use super::{
    cnst::{NOT_OBVIOUS, TODO},
    t::Comment,
};

pub fn not_obvious(comment: &Comment) -> bool {
    _body(comment).starts_with(NOT_OBVIOUS)
}

pub fn todo(comment: &Comment) -> bool {
    _body(comment).starts_with(TODO)
}

fn _body(comment: &Comment) -> &str {
    _inner(&comment.full, comment.is_block).trim_start()
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
