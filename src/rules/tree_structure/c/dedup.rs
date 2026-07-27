use std::collections::HashSet;

use crate::rules::tree_structure::t::SurfaceItem;

pub fn unique(items: Vec<SurfaceItem>) -> Vec<SurfaceItem> {
    let mut seen: HashSet<(String, &'static str, String)> = HashSet::new();
    items
        .into_iter()
        .filter(|i| seen.insert((i.file.display().to_string(), i.kind, i.name.clone())))
        .collect()
}
