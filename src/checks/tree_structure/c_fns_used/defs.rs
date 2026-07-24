use std::path::Path;

use crate::{
    c::{ast, files, path},
    s::EXISTING_SRC_DIRS,
};

use super::t::CFn;

pub fn find() -> Vec<CFn> {
    let mut ret = Vec::new();
    for dir_name in EXISTING_SRC_DIRS.iter() {
        for file in files::rs(dir_name) {
            _collect(&file, &mut ret);
        }
    }
    ret
}

fn _collect(file: &Path, ret: &mut Vec<CFn>) {
    if !path::in_common(file, "c") {
        return;
    }
    let Some(module) = path::module(file) else {
        return;
    };
    let Some(parent) = _parent(&module) else {
        return;
    };
    for item in files::ast_parse(file).items {
        let syn::Item::Fn(f) = item else {
            continue;
        };
        if ast::is_public(&f.vis) {
            ret.push(CFn {
                name: f.sig.ident.to_string(),
                module: module.clone(),
                parent: parent.clone(),
                path: file.to_path_buf(),
            });
        }
    }
}

fn _parent(module: &[String]) -> Option<Vec<String>> {
    let idx = module.iter().position(|s| s == "c")?;
    Some(module[..idx].to_vec())
}
