use crate::s::FILE_CONFIG;

use super::words;

pub fn call(segments: &[String]) -> bool {
    let called = words::significant(segments);
    FILE_CONFIG
        .calls
        .ignore
        .iter()
        .any(|entry| _matches(entry, &called))
}

fn _matches(entry: &str, called: &[&str]) -> bool {
    let wanted: Vec<&str> = entry.split("::").filter(|s| !s.is_empty()).collect();
    if wanted.is_empty() || wanted.len() > called.len() {
        return false;
    }
    called[called.len() - wanted.len()..] == wanted[..]
}
