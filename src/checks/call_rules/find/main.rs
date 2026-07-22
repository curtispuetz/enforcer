use std::collections::HashMap;

use super::{calls, imports, message, words};

pub fn violations(file: &syn::File) -> Vec<String> {
    let imported = imports::bindings(file);
    let mut items = Vec::new();
    for segments in calls::paths(file) {
        if let Some(item) = _item(&segments, &imported) {
            items.push(item);
        }
    }
    items
}

fn _item(segments: &[String], imported: &HashMap<String, Vec<String>>) -> Option<String> {
    let name = segments.last()?;
    if !words::is_function_like(name) {
        return None;
    }
    if segments.len() == 1 {
        return _direct_import(name, imported);
    }
    let word = words::duplicate(segments)?;
    Some(message::repeated_word(segments, &word))
}

fn _direct_import(name: &str, imported: &HashMap<String, Vec<String>>) -> Option<String> {
    let path = imported.get(name)?;
    Some(message::direct_import(name, path))
}
