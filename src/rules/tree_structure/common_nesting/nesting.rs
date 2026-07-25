use std::path::{Path, PathBuf};

use crate::{
    rules::{c::path, t::Results, tree_structure::c::modules},
    t::ItemsViolation,
};

pub fn run() -> Results<ItemsViolation> {
    let mut r = Results::new();
    for module in modules::common() {
        match _bad_ancestor(&module) {
            Some(ancestor) => r.violations.push(ItemsViolation {
                path: path::rel(&module),
                items: vec![format!(
                    "nested inside common module `{}`",
                    path::rel(&ancestor)
                )],
            }),
            None => r.passed += 1,
        }
    }
    r
}

fn _bad_ancestor(module: &Path) -> Option<PathBuf> {
    let kind = modules::common_kind(module)?;
    let (ancestor, ancestor_kind) = modules::ancestor(module)?;
    // not-obvious: c-in-c is the only nesting allowed, so it's the one pairing we skip.
    if kind == "c" && ancestor_kind == "c" {
        return None;
    }
    Some(ancestor)
}
