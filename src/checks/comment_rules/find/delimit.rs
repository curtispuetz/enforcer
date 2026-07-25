pub fn line_end(chars: &[char], i: usize) -> usize {
    let mut j = i + 2;
    while j < chars.len() && chars[j] != '\n' {
        j += 1;
    }
    j
}

pub fn block_end(chars: &[char], i: usize) -> usize {
    let mut depth: i32 = 1;
    let mut j = i + 2;
    while j < chars.len() {
        match _depth_change(chars, j) {
            Some(change) => {
                depth += change;
                j += 2;
                if depth == 0 {
                    return j;
                }
            }
            None => j += 1,
        }
    }
    j
}

fn _depth_change(chars: &[char], j: usize) -> Option<i32> {
    match (chars[j], chars.get(j + 1).copied()) {
        ('/', Some('*')) => Some(1),
        ('*', Some('/')) => Some(-1),
        _ => None,
    }
}
