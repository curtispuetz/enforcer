use crate::rules::comments::t::Comment;

use super::{delimit, group, literal, locate};

pub fn comments(source: &str) -> Vec<Comment> {
    let chars: Vec<char> = source.chars().collect();
    let mut found = Vec::new();
    let mut i = 0;
    while i < chars.len() {
        i = _step(&chars, i, &mut found);
    }
    group::merge(found)
}

fn _step(chars: &[char], i: usize, found: &mut Vec<Comment>) -> usize {
    let next = chars.get(i + 1).copied();
    match chars[i] {
        '/' if next == Some('/') => {
            _emit(chars, i, delimit::line_end(chars, i), false, found)
        }
        '/' if next == Some('*') => {
            _emit(chars, i, delimit::block_end(chars, i), true, found)
        }
        '"' => literal::string_end(chars, i),
        '\'' => literal::char_end(chars, i),
        'r' if next == Some('"') || next == Some('#') => {
            literal::raw_end(chars, i).unwrap_or(i + 1)
        }
        'b' if next == Some('"') => literal::string_end(chars, i + 1),
        'b' if next == Some('\'') => literal::char_end(chars, i + 1),
        'b' if next == Some('r') => literal::raw_end(chars, i + 1).unwrap_or(i + 1),
        _ => i + 1,
    }
}

fn _emit(
    chars: &[char],
    start: usize,
    end: usize,
    is_block: bool,
    found: &mut Vec<Comment>,
) -> usize {
    let full: String = chars[start..end].iter().collect();
    found.push(Comment {
        line: locate::line_at(chars, start),
        col: locate::col(chars, start),
        full: full.trim_end().to_string(),
        is_block,
        trailing: locate::trailing(chars, start),
    });
    end
}
