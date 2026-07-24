use std::{
    collections::{HashMap, HashSet},
    path::Path,
};

use crate::{
    c::{calls, files, imports, path},
    s::EXISTING_SRC_DIRS,
};

use super::{resolve, t::CfKey};

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
    for segments in calls::paths(&ast) {
        let Some(key) = resolve::call_target(&segments, &bindings, file) else {
            continue;
        };
        if defs.contains(&key) {
            ret.entry(key).or_default().push(caller.clone());
        }
    }
}
