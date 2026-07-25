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
    let module = _absolute(&full[..full.len().checked_sub(1)?], file)?;
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

fn _absolute(module: &[String], file: &Path) -> Option<Vec<String>> {
    let mut acc: Vec<String> = Vec::new();
    for (i, seg) in module.iter().enumerate() {
        match seg.as_str() {
            "crate" if i == 0 => acc = vec!["crate".to_string()],
            "self" if i == 0 => acc = path::module(file)?,
            "super" => {
                if i == 0 {
                    acc = path::module(file)?;
                }
                acc.pop()?;
            }
            _ => acc.push(seg.clone()),
        }
    }
    (acc.first().map(String::as_str) == Some("crate")).then_some(acc)
}
