pub fn string_end(chars: &[char], i: usize) -> usize {
    let mut j = i + 1;
    while j < chars.len() {
        match chars[j] {
            '\\' => j += 2,
            '"' => return j + 1,
            _ => j += 1,
        }
    }
    j
}

pub fn char_end(chars: &[char], i: usize) -> usize {
    if chars.get(i + 1) == Some(&'\\') {
        return _escaped_char_end(chars, i);
    }
    if chars.get(i + 2) == Some(&'\'') {
        return i + 3;
    }
    i + 1
}

pub fn raw_end(chars: &[char], r: usize) -> Option<usize> {
    let mut hashes = 0;
    let mut j = r + 1;
    while chars.get(j) == Some(&'#') {
        hashes += 1;
        j += 1;
    }
    if chars.get(j) != Some(&'"') {
        return None;
    }
    j += 1;
    while j < chars.len() {
        if chars[j] == '"' && _closing_hashes(chars, j + 1, hashes) {
            return Some(j + 1 + hashes);
        }
        j += 1;
    }
    Some(chars.len())
}

fn _escaped_char_end(chars: &[char], i: usize) -> usize {
    let mut j = i + 1;
    while j < chars.len() {
        match chars[j] {
            '\\' => j += 2,
            '\'' => return j + 1,
            _ => j += 1,
        }
    }
    j
}

fn _closing_hashes(chars: &[char], start: usize, hashes: usize) -> bool {
    for k in 0..hashes {
        if chars.get(start + k) != Some(&'#') {
            return false;
        }
    }
    true
}
