use {
    crate::rules::t::{ChildLinks, Importer},
    std::collections::BTreeSet,
};

pub fn closure(start: &ChildLinks, links: &[ChildLinks]) -> BTreeSet<String> {
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

pub fn owned(owner: &ChildLinks, candidates: &[&ChildLinks]) -> BTreeSet<String> {
    let mut owned: BTreeSet<String> = candidates
        .iter()
        .filter(|link| link.name != owner.name)
        .map(|link| link.name.clone())
        .collect();
    loop {
        let kept: BTreeSet<String> = candidates
            .iter()
            .filter(|link| {
                owned.contains(&link.name) && _ownable(link, &owner.name, &owned)
            })
            .map(|link| link.name.clone())
            .collect();
        if kept.len() == owned.len() {
            return owned;
        }
        owned = kept;
    }
}

fn _ownable(link: &ChildLinks, owner: &str, owned: &BTreeSet<String>) -> bool {
    let inside = |name: &String| name == owner || owned.contains(name);
    !link.importers.is_empty()
        && link.imports.iter().all(inside)
        && link
            .importers
            .iter()
            .all(|importer| matches!(importer, Importer::Sibling(name) if inside(name)))
}
