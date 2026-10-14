use {
    super::{folder, place},
    crate::rules::{
        c::room,
        file_sizes::t::Place,
        t::{ChildLinks, FileImports, Room},
    },
    std::path::Path,
};

pub fn fixes(path: &Path, index: &[FileImports]) -> Vec<String> {
    let file = path.file_name().unwrap_or_default().to_string_lossy();
    match place::of(path, index) {
        Place::Declared {
            mod_rs,
            name,
            links,
        } => _declared(path, &mod_rs, &name, &links, index),
        Place::Parent { mod_rs, mods } => vec![room::noted(
            &format!("move part of {file} into a new module"),
            &mod_rs,
            &room::of(&mod_rs, mods),
        )],
        Place::Loose => vec![format!(
            "split {file} into smaller modules (no mod.rs or lib.rs declares it, so slots aren't checked)"
        )],
    }
}

fn _declared(
    path: &Path,
    mod_rs: &Path,
    name: &str,
    links: &[ChildLinks],
    index: &[FileImports],
) -> Vec<String> {
    let room = room::of(mod_rs, links.len());
    let sibling = room::noted(
        &format!("move part of {name}.rs into a new sibling file"),
        mod_rs,
        &room,
    );
    let mut lines = folder::lines(path, mod_rs, name, links, index);
    let at = if matches!(room, Room::Full) {
        lines.len()
    } else {
        0
    };
    lines.insert(at, sibling);
    lines
}
