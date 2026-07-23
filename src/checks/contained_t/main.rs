use std::{collections::HashMap, path::Path};

use crate::{
    c::{files, path, scan},
    t::{ItemsViolation, Outcome},
};

use super::{calls, imports, report};

pub fn run() -> bool {
    let (passed, violations) = scan::src_files(_check_file);
    report::print(passed, violations)
}

fn _check_file(path: &Path) -> Outcome<ItemsViolation> {
    if !_is_t_commons(path) {
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

// A call violates the rule when it targets a free function that lives in our crate but
// in a different module than this t module. Associated functions (`Type::f()`), calls
// into dependencies/std, and calls to functions defined in this same module are fine.
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
        // A bare `foo()` is only a violation when a `use` pulled it in from elsewhere in
        // our crate; otherwise it is defined in this same module or comes from std.
        1 => imported.get(name).is_some_and(|p| _is_internal_path(p)),
        _ => {
            // `Type::f()` is an associated function, not a free function.
            if _is_type_segment(&segments[segments.len() - 2]) {
                return false;
            }
            // `self::f()` stays inside this module (the same-module exception).
            let root = &segments[0];
            root != "self" && _is_internal(root, imported)
        }
    }
}

fn _is_internal(root: &str, imported: &HashMap<String, Vec<String>>) -> bool {
    matches!(root, "crate" | "super")
        || imported.get(root).is_some_and(|path| _is_internal_path(path))
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

fn _is_t_commons(path: &Path) -> bool {
    path::under_dir(path, "t") || path::commons_file_kind(path) == Some("t")
}
