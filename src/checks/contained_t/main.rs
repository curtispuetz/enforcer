use std::{collections::HashMap, path::Path};

use crate::{
    c::{calls, files, imports, path, scan},
    t::{ItemsViolation, Outcome},
};

use super::report;

pub fn run() -> bool {
    let (passed, violations) = scan::src_files(_check_file);
    report::print(passed, violations)
}

fn _check_file(path: &Path) -> Outcome<ItemsViolation> {
    if !path::is_t_commons(path) {
        return Outcome::Skipped;
    }
    let file = files::parse(path);
    let imported = imports::bindings(&file);
    let mut items = Vec::new();
    for segments in calls::paths(&file) {
        if _is_foreign_free_call(&segments, &imported) {
            items.push(format!("{}()", segments.join("::")));
        }
    }
    if items.is_empty() {
        Outcome::Passed
    } else {
        Outcome::Failed(ItemsViolation {
            path: path::rel(path),
            items,
        })
    }
}

fn _is_foreign_free_call(
    segments: &[String],
    imported: &HashMap<String, Vec<String>>,
) -> bool {
    let Some(name) = segments.last() else {
        return false;
    };
    if !_is_function_like(name) {
        return false;
    }
    match segments.len() {
        // not-obvious: a bare call no `use` imported must be defined in this same
        // module (or be a std/prelude name) — either way it is exempt.
        1 => imported.get(name).is_some_and(|p| _is_internal_path(p)),
        _ => {
            let parent = &segments[segments.len() - 2];
            let root = &segments[0];
            !_is_type_segment(parent) && root != "self" && _is_internal(root, imported)
        }
    }
}

fn _is_internal(root: &str, imported: &HashMap<String, Vec<String>>) -> bool {
    matches!(root, "crate" | "super")
        || imported
            .get(root)
            .is_some_and(|path| _is_internal_path(path))
}

fn _is_internal_path(path: &[String]) -> bool {
    matches!(
        path.first().map(String::as_str),
        Some("crate" | "super" | "self")
    )
}

fn _is_function_like(name: &str) -> bool {
    matches!(name.chars().next(), Some(c) if c == '_' || c.is_ascii_lowercase())
}

fn _is_type_segment(segment: &str) -> bool {
    matches!(segment.chars().next(), Some(c) if c.is_ascii_uppercase())
}
