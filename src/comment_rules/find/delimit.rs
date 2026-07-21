pub fn line_end(chars: &[char], i: usize) -> usize {
    let mut j = i + 2;
    while j < chars.len() && chars[j] != '\n' {
        j += 1;
    }
    j
}

pub fn block_end(chars: &[char], i: usize) -> usize {
    let mut depth = 1;
    let mut j = i + 2;
    while j < chars.len() {
        if chars[j] == '/' && chars.get(j + 1) == Some(&'*') {
            depth += 1;
            j += 2;
        } else if chars[j] == '*' && chars.get(j + 1) == Some(&'/') {
            depth -= 1;
            j += 2;
            if depth == 0 {
                return j;
            }
        } else {
            j += 1;
        }
    }
    j
}
