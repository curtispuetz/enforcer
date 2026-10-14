use {
    crate::rules::{
        c::{child, nesting},
        mod_count::t::Fix,
        t::{ChildLinks, Importer},
    },
    std::{collections::BTreeSet, path::Path},
};

pub fn all(mod_rs: &Path, links: &[ChildLinks]) -> Vec<Fix> {
    let mut fixes = Vec::new();
    for link in child::movable(links) {
        let members = nesting::closure(link, links);
        if members.len() > 1 && !members.contains("main") {
            fixes.push(Fix::Group {
                repoint: _repoint(mod_rs, &members, links),
                members: members.into_iter().collect(),
            });
        }
    }
    fixes
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
        other => Some(child::importer(mod_rs, other)),
    }
}
