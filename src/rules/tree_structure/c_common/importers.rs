use {
    super::t::{Folder, Member},
    crate::rules::{s::COMMON, t::FileImports},
};

pub fn of<'a>(
    folder: &Folder,
    member: &Member,
    index: &'a [FileImports],
) -> Vec<&'a FileImports> {
    index
        .iter()
        .filter(|file| _imports(folder, member, file))
        .collect()
}

fn _imports(folder: &Folder, member: &Member, file: &FileImports) -> bool {
    !file.module.starts_with(&member.segments)
        && file.module != folder.segments
        && file.targets.iter().any(|t| _hits(folder, member, t))
}

// not-obvious: a path that stops at the `c` folder, or names something its mod.rs
// re-exports, can't be pinned to one member, so it counts for all of them.
fn _hits(folder: &Folder, member: &Member, target: &[String]) -> bool {
    if target.starts_with(&member.segments) {
        return true;
    }
    target.starts_with(&folder.segments)
        && target
            .get(folder.segments.len())
            .is_none_or(|name| !_is_member_or_common(folder, name))
}

fn _is_member_or_common(folder: &Folder, name: &str) -> bool {
    COMMON.contains(&name) || folder.members.iter().any(|m| m.name == name)
}
