use {
    super::child,
    crate::rules::{
        c::path,
        mod_count::t::Fix,
        t::{ChildLinks, Importer},
    },
    std::{collections::BTreeSet, path::Path},
};

pub fn all(mod_rs: &Path, links: &[ChildLinks]) -> Vec<Fix> {
    let mut fixes = Vec::new();
    for link in child::movable(links) {
        let members = _closure(link, links);
        if members.len() > 1 && !members.contains("main") {
            fixes.push(Fix::Group {
                repoint: _repoint(mod_rs, &members, links),
                members: members.into_iter().collect(),
            });
        }
    }
    fixes
}

fn _closure(start: &ChildLinks, links: &[ChildLinks]) -> BTreeSet<String> {
    let mut members = BTreeSet::from([start.name.clone()]);
    let mut todo = vec![start];
    while let Some(next) = todo.pop() {
        for name in &next.imports {
            if members.insert(name.clone()) {
                todo.extend(links.iter().find(|link| &link.name == name));
            }
        }
    }
    members
}

fn _repoint(
    mod_rs: &Path,
    members: &BTreeSet<String>,
    links: &[ChildLinks],
) -> Vec<String> {
    let outside: BTreeSet<String> = links
        .iter()
        .filter(|link| members.contains(&link.name))
        .flat_map(|link| &link.importers)
        .filter_map(|importer| _outside(mod_rs, importer, members))
        .collect();
    outside.into_iter().collect()
}

fn _outside(
    mod_rs: &Path,
    importer: &Importer,
    members: &BTreeSet<String>,
) -> Option<String> {
    match importer {
        Importer::Sibling(name) if members.contains(name) => None,
        Importer::Sibling(name) => Some(name.clone()),
        Importer::Parent => Some(path::rel(mod_rs)),
        Importer::Elsewhere(file) => Some(path::rel(file)),
    }
}
