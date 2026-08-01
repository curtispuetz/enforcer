use std::path::{Path, PathBuf};

use crate::rules::tree_structure::c::{modules, path, surface};

use super::t::{CFn, Defs, Reach};

pub fn find() -> Defs {
    let mut ret = Defs {
        cfns: Vec::new(),
        reaches: Vec::new(),
    };
    for module in modules::common() {
        _collect(&module, &mut ret);
    }
    ret
}

fn _collect(module: &Path, ret: &mut Defs) {
    if modules::common_kind(module) != Some("c") {
        return;
    }
    let Some(parent) = _parent(module) else {
        return;
    };
    for item in surface::items(module) {
        if item.kind != "fn" {
            continue;
        }
        let key = (item.module, item.name);
        _push(ret, key, item.file, module, &parent);
    }
}

// not-obvious: a fn re-exported up out of a nested `c` is owned by that nested
// module (judged against its own parent); the outer module's path to it only
// records another way call sites can reach it.
fn _push(
    ret: &mut Defs,
    key: (Vec<String>, String),
    path: PathBuf,
    module: &Path,
    parent: &[String],
) {
    if _home(&path).as_deref() != Some(module) {
        ret.reaches.push(Reach { key, path });
        return;
    }
    ret.cfns.push(CFn {
        name: key.1,
        module: key.0,
        parent: parent.to_vec(),
        path,
    });
}

fn _home(file: &Path) -> Option<PathBuf> {
    if modules::common_kind(file) == Some("c") {
        return Some(file.to_path_buf());
    }
    match modules::ancestor(file) {
        Some((dir, "c")) => Some(dir),
        _ => None,
    }
}

fn _parent(module: &Path) -> Option<Vec<String>> {
    let mut segments = path::module(&modules::root_file(module))?;
    segments.pop()?;
    Some(segments)
}
