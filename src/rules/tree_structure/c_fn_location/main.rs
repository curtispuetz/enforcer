use std::collections::{HashMap, HashSet};

use crate::rules::{
    c::path,
    tree_structure::t::{FileViolation, PartReport},
};

use super::{
    defs, judge, keys, reexports,
    t::{CFn, CfKey, Defs, Reach},
    usage,
};

pub fn part() -> PartReport {
    let aliases = reexports::aliases();
    let defs = _canonical(defs::find(), &aliases);
    let owners = keys::owners(&defs);
    let callers = usage::collect(&owners, &aliases);
    _report(defs.cfns, &callers)
}

fn _canonical(defs: Defs, aliases: &HashSet<Vec<String>>) -> Defs {
    Defs {
        cfns: defs.cfns.into_iter().map(|c| _cfn(c, aliases)).collect(),
        reaches: defs
            .reaches
            .into_iter()
            .map(|r| _reach(r, aliases))
            .collect(),
    }
}

fn _cfn(cfn: CFn, aliases: &HashSet<Vec<String>>) -> CFn {
    CFn {
        module: reexports::canonical(&cfn.module, aliases),
        parent: reexports::canonical(&cfn.parent, aliases),
        ..cfn
    }
}

fn _reach(reach: Reach, aliases: &HashSet<Vec<String>>) -> Reach {
    Reach {
        key: (reexports::canonical(&reach.key.0, aliases), reach.key.1),
        ..reach
    }
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
