use std::{
    collections::{HashMap, HashSet},
    path::Path,
};

use crate::{
    c::{files, imports, path},
    s::EXISTING_SRC_DIRS,
};

use super::{resolve, t::CfKey, visit};

pub fn collect(defs: &HashSet<CfKey>) -> HashMap<CfKey, Vec<Vec<String>>> {
    let mut ret: HashMap<CfKey, Vec<Vec<String>>> = HashMap::new();
    for dir_name in EXISTING_SRC_DIRS.iter() {
        for file in files::rs(dir_name) {
            _scan(&file, defs, &mut ret);
        }
    }
    ret
}

fn _scan(
    file: &Path,
    defs: &HashSet<CfKey>,
    ret: &mut HashMap<CfKey, Vec<Vec<String>>>,
) {
    let ast = files::ast_parse(file);
    let bindings = imports::bindings(&ast);
    let Some(caller) = path::module(file) else {
        return;
    };
    for (scope, segments) in visit::scoped(&ast) {
        let Some(key) = resolve::call_target(&segments, &bindings, file) else {
            continue;
        };
        if _recursive(&scope, &key, &caller) {
            continue;
        }
        if defs.contains(&key) {
            ret.entry(key).or_default().push(caller.clone());
        }
    }
}

fn _recursive(scope: &Option<String>, key: &CfKey, caller: &[String]) -> bool {
    scope.as_deref() == Some(key.1.as_str()) && key.0.as_slice() == caller
}
