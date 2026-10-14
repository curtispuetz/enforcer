use {
    super::t::{Folder, Member},
    crate::rules::{s::COMMON, t::FileImports},
    std::collections::HashSet,
};

pub fn unneeded<'a>(folder: &'a Folder, index: &[FileImports]) -> HashSet<&'a str> {
    let mut flagged = HashSet::new();
    while _pass(folder, index, &mut flagged) {}
    flagged
}

pub fn importers<'a>(
    folder: &Folder,
    member: &Member,
    index: &'a [FileImports],
) -> Vec<&'a FileImports> {
    index
        .iter()
        .filter(|file| _imports(folder, member, file))
        .collect()
}

fn _pass<'a>(
    folder: &'a Folder,
    index: &[FileImports],
    flagged: &mut HashSet<&'a str>,
) -> bool {
    let mut changed = false;
    for member in &folder.members {
        if !flagged.contains(member.name.as_str()) && _movable(folder, member, index, flagged) {
            flagged.insert(member.name.as_str());
            changed = true;
        }
    }
    changed
}

fn _movable(
    folder: &Folder,
    member: &Member,
    index: &[FileImports],
    flagged: &HashSet<&str>,
) -> bool {
    let importers = importers(folder, member, index);
    !importers.is_empty()
        && importers.iter().all(|file| _beside(folder, file, flagged))
        && !_reaches_inner_common(folder, member, index)
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

// not-obvious: a member file directly in the `c` folder counts as beside it once
// that member is itself flagged, since both would move up together.
fn _beside(folder: &Folder, file: &FileImports, flagged: &HashSet<&str>) -> bool {
    if file.dir == folder.parent {
        return true;
    }
    file.dir == folder.segments
        && file.module.last().is_some_and(|name| flagged.contains(name.as_str()))
}

fn _reaches_inner_common(folder: &Folder, member: &Member, index: &[FileImports]) -> bool {
    let depth = folder.segments.len();
    index
        .iter()
        .filter(|file| file.module.starts_with(&member.segments))
        .flat_map(|file| &file.targets)
        .any(|t| {
            t.starts_with(&folder.segments)
                && t.get(depth).is_some_and(|name| COMMON.contains(&name.as_str()))
        })
}
