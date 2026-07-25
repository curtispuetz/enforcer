use std::collections::{HashMap, HashSet};

use crate::rules::{
    c::path,
    tree_structure::t::{FileViolation, PartReport},
};

use super::{
    defs, judge, reexports,
    t::{CFn, CfKey},
    usage,
};

pub fn part() -> PartReport {
    let aliases = reexports::aliases();
    let cfns = defs::find();
    let keys: HashSet<CfKey> = cfns
        .iter()
        .map(|f| (f.module.clone(), f.name.clone()))
        .collect();
    let callers = usage::collect(&keys, &aliases);
    _report(cfns, &callers)
}

fn _report(cfns: Vec<CFn>, callers: &HashMap<CfKey, Vec<Vec<String>>>) -> PartReport {
    let empty = Vec::new();
    let mut files: HashSet<String> = HashSet::new();
    let mut lines: HashMap<String, Vec<String>> = HashMap::new();
    for cfn in &cfns {
        let rel = path::rel(&cfn.path);
        files.insert(rel.clone());
        let reached = callers.get(&(cfn.module.clone(), cfn.name.clone()));
        if let Some(line) = judge::reason(cfn, reached.unwrap_or(&empty)) {
            lines.entry(rel).or_default().push(line);
        }
    }
    _build(files.len(), lines)
}

fn _build(total: usize, lines: HashMap<String, Vec<String>>) -> PartReport {
    let failing = lines.len();
    let mut violations: Vec<FileViolation> = lines
        .into_iter()
        .map(|(path, mut items)| {
            items.sort();
            FileViolation { path, lines: items }
        })
        .collect();
    violations.sort_by(|a, b| a.path.cmp(&b.path));
    PartReport {
        name: "c-fn-location",
        unit: "c files",
        passed: total - failing,
        violations,
    }
}
