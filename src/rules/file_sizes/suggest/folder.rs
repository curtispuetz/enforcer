use {
    super::blocked,
    crate::rules::{
        c::{child, nesting, path, room},
        s::COMMON,
        t::{ChildLinks, FileImports},
    },
    std::{collections::BTreeSet, path::Path},
};

pub fn lines(
    path: &Path,
    mod_rs: &Path,
    name: &str,
    links: &[ChildLinks],
    index: &[FileImports],
) -> Vec<String> {
    let Some(owner) = links.iter().find(|link| link.name == name) else {
        return Vec::new();
    };
    let reach = nesting::closure(owner, links);
    let candidates: Vec<&ChildLinks> = child::movable(links)
        .into_iter()
        .filter(|link| reach.contains(&link.name))
        .collect();
    let owned = nesting::owned(owner, &candidates);
    let blocked: Vec<&ChildLinks> = links
        .iter()
        .filter(|link| owner.imports.contains(&link.name) && !owned.contains(&link.name))
        .collect();
    let head = format!("turn {name}.rs into folder {name}/ (mod.rs re-exporting main.rs)");
    if !blocked.is_empty() {
        return blocked::lines(&head, mod_rs, owner, &owned, &blocked);
    }
    vec![format!(
        "{head}: clean, {}{}",
        _nested(mod_rs, name, &owned),
        _commons(path, mod_rs, index)
    )]
}

fn _nested(mod_rs: &Path, name: &str, owned: &BTreeSet<String>) -> String {
    if owned.is_empty() {
        return "it imports no plain siblings".to_string();
    }
    let labels: Vec<String> = owned.iter().map(|n| child::label(mod_rs, n)).collect();
    format!(
        "nest {} along into {name}/ (frees {} in {})",
        labels.join(", "),
        room::slots(owned.len()),
        path::rel(mod_rs)
    )
}

fn _commons(path: &Path, mod_rs: &Path, index: &[FileImports]) -> String {
    let Some(folder) = path::module(mod_rs) else {
        return String::new();
    };
    let used: BTreeSet<String> = index
        .iter()
        .filter(|file| file.path == path)
        .flat_map(|file| &file.targets)
        .filter_map(|target| target.strip_prefix(folder.as_slice())?.first())
        .filter(|name| COMMON.contains(&name.as_str()))
        .map(|name| format!("super::{name}"))
        .collect();
    if used.is_empty() {
        return String::new();
    }
    let used: Vec<String> = used.into_iter().collect();
    format!("; its {} imports become crate:: paths", used.join(", "))
}
