use {
    crate::rules::{
        c::{path, siblings},
        file_sizes::t::Place,
        t::FileImports,
    },
    std::path::{Path, PathBuf},
};

pub fn of(path: &Path, index: &[FileImports]) -> Place {
    let Some(mod_rs) = _mod_rs(path) else {
        return Place::Loose;
    };
    let links = siblings::of(&mod_rs, index);
    let name = path.file_stem().unwrap_or_default().to_string_lossy();
    if mod_rs != path && links.iter().any(|link| link.name == name) {
        return Place::Declared {
            mod_rs,
            name: name.to_string(),
            links,
        };
    }
    if mod_rs == path || (name == "main" && mod_rs.ends_with("lib.rs")) {
        return Place::Parent {
            mod_rs,
            mods: links.len(),
        };
    }
    Place::Loose
}

fn _mod_rs(path: &Path) -> Option<PathBuf> {
    if path::rel(path).starts_with("tests/") {
        return None;
    }
    if path::is_mod_or_lib(path) {
        return Some(path.to_path_buf());
    }
    let dir = path.parent()?;
    ["mod.rs", "lib.rs"]
        .iter()
        .map(|file| dir.join(file))
        .find(|file| file.is_file())
}
