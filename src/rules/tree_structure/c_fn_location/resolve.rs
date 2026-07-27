use std::{collections::HashMap, path::Path};

use crate::rules::{c::calls, tree_structure::c::path};

use super::t::CfKey;

pub fn call_target(
    segments: &[String],
    bindings: &HashMap<String, Vec<String>>,
    file: &Path,
) -> Option<CfKey> {
    let name = segments.last()?.clone();
    if !calls::is_function_like(&name) {
        return None;
    }
    if segments.len() == 1 && !bindings.contains_key(&name) {
        return Some((path::module(file)?, name));
    }
    let full = _expand(segments, bindings)?;
    let module = path::absolute(&full[..full.len().checked_sub(1)?], file)?;
    Some((module, name))
}

fn _expand(
    segments: &[String],
    bindings: &HashMap<String, Vec<String>>,
) -> Option<Vec<String>> {
    let first = segments.first()?;
    if matches!(first.as_str(), "crate" | "super" | "self") {
        return Some(segments.to_vec());
    }
    if segments.len() == 1 {
        return bindings.get(first).cloned();
    }
    let mut full = bindings.get(first)?.clone();
    full.extend_from_slice(&segments[1..]);
    Some(full)
}
