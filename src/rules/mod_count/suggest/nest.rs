use {
    crate::rules::{
        c::{child, exports, files, nesting},
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

fn _mods(mod_rs: &Path) -> usize {
    exports::table(mod_rs, &files::ast_parse(mod_rs))
        .children
        .len()
}
