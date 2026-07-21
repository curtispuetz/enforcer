pub fn line_at(chars: &[char], start: usize) -> usize {
    let mut line = 1;
    for &c in &chars[..start] {
        if c == '\n' {
            line += 1;
        }
    }
    line
}

pub fn trailing(chars: &[char], start: usize) -> bool {
    let mut j = start;
    while j > 0 {
        j -= 1;
        let c = chars[j];
        if c == '\n' {
            return false;
        }
        if !c.is_whitespace() {
            return true;
        }
    }
    false
}
