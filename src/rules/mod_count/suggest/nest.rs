use {
    super::child,
    crate::rules::{
        c::{exports, files},
        mod_count::t::Fix,
        t::{ChildLinks, Importer},
    },
    std::{collections::BTreeSet, path::Path},
};

pub fn all(mod_rs: &Path, links: &[ChildLinks], max: usize) -> Vec<Fix> {
    let movable = child::movable(links);
    movable
        .iter()
        .filter_map(|owner| _nest(mod_rs, owner, &movable, max))
        .collect()
}

fn _nest(
    mod_rs: &Path,
    owner: &ChildLinks,
    movable: &[&ChildLinks],
    max: usize,
) -> Option<Fix> {
    let owned = _owned(owner, movable);
    if owned.is_empty() {
        return None;
    }
    let room = match (
        child::file(mod_rs, &owner.name),
        child::folder(mod_rs, &owner.name),
    ) {
        (_, Some(owner_rs)) => Some(max.saturating_sub(_mods(&owner_rs))),
        (Some(_), None) if owner.imports.is_subset(&owned) => None,
        _ => return None,
    };
    Some(Fix::Nest {
        owner: owner.name.clone(),
        owned: owned.into_iter().collect(),
        room,
    })
}

fn _owned(owner: &ChildLinks, movable: &[&ChildLinks]) -> BTreeSet<String> {
    let mut owned: BTreeSet<String> = movable
        .iter()
        .filter(|link| link.name != owner.name)
        .map(|link| link.name.clone())
        .collect();
    loop {
        let kept: BTreeSet<String> = movable
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

fn _mods(mod_rs: &Path) -> usize {
    exports::table(mod_rs, &files::ast_parse(mod_rs))
        .children
        .len()
}
