use std::collections::HashMap;

use crate::rules::{
    c::path,
    tree_structure::c::{modules, surface},
};

pub fn disallowed() -> HashMap<String, Vec<String>> {
    let mut ret: HashMap<String, Vec<String>> = HashMap::new();
    for module in modules::common() {
        _check(&module, &mut ret);
    }
    for items in ret.values_mut() {
        items.sort();
        items.dedup();
    }
    ret
}

fn _check(module: &std::path::Path, ret: &mut HashMap<String, Vec<String>>) {
    let Some(common) = modules::common_kind(module) else {
        return;
    };
    for item in surface::items(module) {
        if !_allowed(common, item.kind) {
            let entry = ret.entry(path::rel(&item.file)).or_default();
            entry.push(format!("{} {}", item.kind, item.name));
        }
    }
}

fn _allowed(common: &str, kind: &str) -> bool {
    match common {
        "t" => matches!(kind, "struct" | "enum" | "type" | "trait"),
        "s" => kind == "static",
        "cnst" => kind == "const",
        "ext_traits" => kind == "trait",
        _ => kind == "fn",
    }
}
