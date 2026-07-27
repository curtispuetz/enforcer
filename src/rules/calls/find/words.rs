use std::collections::HashSet;

pub fn duplicate(segments: &[String]) -> Option<String> {
    let mut seen = HashSet::new();
    _words(segments)
        .into_iter()
        .find(|word| !seen.insert(word.clone()))
}

pub fn significant(segments: &[String]) -> Vec<&str> {
    segments
        .iter()
        .map(String::as_str)
        .filter(|segment| !_is_keyword(segment))
        .collect()
}

fn _words(segments: &[String]) -> Vec<String> {
    let mut ret = Vec::new();
    for segment in significant(segments) {
        for part in segment.split('_') {
            if !part.is_empty() {
                ret.push(part.to_ascii_lowercase());
            }
        }
    }
    ret
}

fn _is_keyword(segment: &str) -> bool {
    matches!(segment, "crate" | "super" | "self" | "Self")
}
