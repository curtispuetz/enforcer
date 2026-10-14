use {super::words, crate::rules::c::suggestions, colored::Colorize};

pub fn direct_import(name: &str, path: &[String]) -> Vec<String> {
    let fix = match path {
        [.., parent, _] => format!("call it as {parent}::{name}()"),
        _ => format!("import a parent module and call it as parent::{name}()"),
    };
    _lines(format!("{name}() is imported directly"), Some(fix))
}

pub fn repeated_word(segments: &[String], word: &str) -> Vec<String> {
    let call = format!("{}()", segments.join("::"));
    let fix = segments.last().zip(words::renamed(segments, word));
    _lines(
        format!("{call} repeats the word '{word}'"),
        fix.map(|(name, new)| format!("rename {name} to {new}")),
    )
}

fn _lines(problem: String, fix: Option<String>) -> Vec<String> {
    let mut lines = vec![problem.red().to_string()];
    let fixes = suggestions::lines(&Vec::from_iter(fix));
    lines.extend(fixes.iter().map(|line| format!("  {line}")));
    lines
}
