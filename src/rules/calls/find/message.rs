pub fn direct_import(name: &str, path: &[String]) -> String {
    if path.len() >= 2 {
        let parent = &path[path.len() - 2];
        format!("{name}() is imported directly; call it as {parent}::{name}() instead")
    } else {
        format!(
            "{name}() is imported directly; import a parent module and call it as parent::{name}()"
        )
    }
}

pub fn repeated_word(segments: &[String], word: &str) -> String {
    let call = format!("{}()", segments.join("::"));
    format!("{call} repeats the word '{word}'")
}
