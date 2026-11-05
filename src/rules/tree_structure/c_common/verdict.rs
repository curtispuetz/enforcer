use {
    super::{
        importers,
        t::{Folder, Member},
    },
    crate::rules::{s::COMMON, t::FileImports},
    std::collections::HashSet,
};

pub fn unneeded<'a>(folder: &'a Folder, index: &[FileImports]) -> HashSet<&'a str> {
    let mut flagged = HashSet::new();
    while _pass(folder, index, &mut flagged) {}
    flagged
}

fn _pass<'a>(
    folder: &'a Folder,
    index: &[FileImports],
    flagged: &mut HashSet<&'a str>,
) -> bool {
    let mut changed = false;
    for member in &folder.members {
        if !flagged.contains(member.name.as_str())
            && _movable(folder, member, index, flagged)
        {
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
    let importers = importers::of(folder, member, index);
    !importers.is_empty()
        && importers.iter().all(|file| _beside(folder, file, flagged))
        && !_reaches_inner_common(folder, member, index)
}

// not-obvious: a member file directly in the `c` folder counts as beside it once
// that member is itself flagged, since both would move up together.
fn _beside(folder: &Folder, file: &FileImports, flagged: &HashSet<&str>) -> bool {
    if file.dir == folder.parent {
        return true;
    }
    file.dir == folder.segments
        && file
            .module
            .last()
            .is_some_and(|name| flagged.contains(name.as_str()))
}

fn _reaches_inner_common(
    folder: &Folder,
    member: &Member,
    index: &[FileImports],
) -> bool {
    let depth = folder.segments.len();
    index
        .iter()
        .filter(|file| file.module.starts_with(&member.segments))
        .flat_map(|file| &file.targets)
        .any(|t| {
            t.starts_with(&folder.segments)
                && t.get(depth)
                    .is_some_and(|name| COMMON.contains(&name.as_str()))
        })
}
