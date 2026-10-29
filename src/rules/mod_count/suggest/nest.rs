use {
    crate::rules::{
        c::{child, count, nesting},
        mod_count::t::Fix,
        t::ChildLinks,
    },
    std::path::Path,
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
    let owned = nesting::owned(owner, movable);
    if owned.is_empty() {
        return None;
    }
    let room = match (
        child::file(mod_rs, &owner.name),
        child::folder(mod_rs, &owner.name),
    ) {
        (_, Some(owner_rs)) => Some(max.saturating_sub(count::mods(&owner_rs))),
        (Some(_), None) if owner.imports.is_subset(&owned) => None,
        _ => return None,
    };
    Some(Fix::Nest {
        owner: owner.name.clone(),
        owned: owned.into_iter().collect(),
        room,
    })
}
