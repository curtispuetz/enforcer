use std::{collections::HashMap, path::Path};

use super::t::{CfKey, Defs};

type ById = HashMap<(String, String), CfKey>;

pub fn owners(defs: &Defs) -> HashMap<CfKey, CfKey> {
    let by_def: ById = defs
        .cfns
        .iter()
        .map(|f| (_id(&f.path, &f.name), (f.module.clone(), f.name.clone())))
        .collect();
    let mut ret: HashMap<CfKey, CfKey> =
        by_def.values().map(|key| (key.clone(), key.clone())).collect();
    for reach in &defs.reaches {
        if let Some(owner) = by_def.get(&_id(&reach.path, &reach.key.1)) {
            ret.insert(reach.key.clone(), owner.clone());
        }
    }
    ret
}

fn _id(path: &Path, name: &str) -> (String, String) {
    (path.display().to_string(), name.to_string())
}
