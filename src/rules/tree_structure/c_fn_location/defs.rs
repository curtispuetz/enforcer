use std::path::Path;

use crate::rules::tree_structure::c::{modules, path, surface};

use super::t::CFn;

pub fn find() -> Vec<CFn> {
    let mut ret = Vec::new();
    for module in modules::common() {
        _collect(&module, &mut ret);
    }
    ret
}

// not-obvious: a `c` nested in a `c` is judged through its outer module, whose
// surface already carries whatever the nested one re-exports.
fn _collect(module: &Path, ret: &mut Vec<CFn>) {
    if modules::common_kind(module) != Some("c") || modules::ancestor(module).is_some()
    {
        return;
    }
    let Some(parent) = _parent(module) else {
        return;
    };
    for item in surface::items(module) {
        if item.kind == "fn" {
            ret.push(CFn {
                name: item.name,
                module: item.module,
                parent: parent.clone(),
                path: item.file,
            });
        }
    }
}

fn _parent(module: &Path) -> Option<Vec<String>> {
    let mut segments = path::module(&modules::root_file(module))?;
    segments.pop()?;
    Some(segments)
}
