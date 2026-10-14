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

pub fn renamed(segments: &[String], word: &str) -> Option<String> {
    let (name, path) = segments.split_last()?;
    let mut spare = !_words(path).iter().any(|w| w == word);
    let kept: Vec<&str> = name
        .split('_')
        .filter(|part| _keep(part, word, &mut spare))
        .collect();
    let new = kept.join("_");
    let valid = new != *name
        && !new.trim_matches('_').is_empty()
        && syn::parse_str::<syn::Ident>(&new).is_ok();
    valid.then_some(new)
}

fn _keep(part: &str, word: &str, spare: &mut bool) -> bool {
    if !part.eq_ignore_ascii_case(word) {
        return true;
    }
    std::mem::replace(spare, false)
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
