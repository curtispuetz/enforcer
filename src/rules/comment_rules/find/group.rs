use crate::rules::comment_rules::t::Comment;

pub fn merge(raw: Vec<Comment>) -> Vec<Comment> {
    let mut out: Vec<Comment> = Vec::new();
    for cur in raw {
        if let Some(prev) = out.last_mut()
            && _continues(prev, &cur)
        {
            prev.full.push('\n');
            prev.full.push_str(&cur.full);
            continue;
        }
        out.push(cur);
    }
    out
}

fn _continues(prev: &Comment, cur: &Comment) -> bool {
    !prev.is_block
        && !cur.is_block
        && !prev.trailing
        && !cur.trailing
        && prev.col == cur.col
        && cur.line == prev.line + _line_count(&prev.full)
}

fn _line_count(full: &str) -> usize {
    full.chars().filter(|&c| c == '\n').count() + 1
}
