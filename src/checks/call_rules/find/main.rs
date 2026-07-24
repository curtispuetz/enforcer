use std::collections::HashMap;
use syn::visit::Visit;

use crate::checks::{
    c::{calls, imports},
    t::CallsCollector,
};

use super::{message, words};

pub fn violations(file: &syn::File) -> Vec<String> {
    let imported = imports::bindings(file);
    let mut items = Vec::new();
    for segments in _paths(file) {
        if let Some(item) = _item(&segments, &imported) {
            items.push(item);
        }
    }
    items
}

fn _item(
    segments: &[String],
    imported: &HashMap<String, Vec<String>>,
) -> Option<String> {
    let name = segments.last()?;
    if !calls::is_function_like(name) {
        return None;
    }
    if segments.len() == 1 {
        return _direct_import(name, imported);
    }
    if !_is_internal(&segments[0], imported) {
        return None;
    }
    let word = words::duplicate(segments)?;
    Some(message::repeated_word(segments, &word))
}

fn _direct_import(
    name: &str,
    imported: &HashMap<String, Vec<String>>,
) -> Option<String> {
    let path = imported.get(name)?;
    if !_is_internal_path(path) {
        return None;
    }
    Some(message::direct_import(name, path))
}

fn _is_internal(root: &str, imported: &HashMap<String, Vec<String>>) -> bool {
    matches!(root, "crate" | "super" | "self")
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

fn _paths(file: &syn::File) -> Vec<Vec<String>> {
    let mut collector = CallsCollector { paths: Vec::new() };
    collector.visit_file(file);
    collector.paths
}
