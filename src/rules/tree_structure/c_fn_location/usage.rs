use std::{
    collections::{HashMap, HashSet},
    path::Path,
};

use crate::{
    rules::c::{files, imports},
    s::EXISTING_SRC_DIRS,
};

use super::{reexports, resolve, t::CfKey, visit};

pub fn collect(
    owners: &HashMap<CfKey, CfKey>,
    aliases: &HashSet<Vec<String>>,
) -> HashMap<CfKey, Vec<Vec<String>>> {
    let mut ret: HashMap<CfKey, Vec<Vec<String>>> = HashMap::new();
    for dir_name in EXISTING_SRC_DIRS.iter() {
        for file in files::rs(dir_name) {
            _scan(&file, owners, aliases, &mut ret);
        }
    }
    ret
}

fn _scan(
    file: &Path,
    owners: &HashMap<CfKey, CfKey>,
    aliases: &HashSet<Vec<String>>,
    ret: &mut HashMap<CfKey, Vec<Vec<String>>>,
) {
    let ast = files::ast_parse(file);
    let bindings = imports::bindings(&ast);
    let Some(caller) = reexports::module_of(file, aliases) else {
        return;
    };
    for (scope, segments) in visit::scoped(&ast) {
        let Some((raw_module, name)) = resolve::call_target(&segments, &bindings, file)
        else {
            continue;
        };
        let key = (reexports::canonical(&raw_module, aliases), name);
        let Some(owner) = owners.get(&key) else {
            continue;
        };
        if _recursive(&scope, owner, &caller) {
            continue;
        }
        ret.entry(owner.clone()).or_default().push(caller.clone());
    }
}

fn _recursive(scope: &Option<String>, key: &CfKey, caller: &[String]) -> bool {
    scope.as_deref() == Some(key.1.as_str()) && key.0.as_slice() == caller
}
