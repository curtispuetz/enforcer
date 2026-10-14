use {
    super::path,
    crate::rules::t::{ChildLinks, Importer},
    std::path::{Path, PathBuf},
};

pub fn movable(links: &[ChildLinks]) -> Vec<&ChildLinks> {
    links
        .iter()
        .filter(|link| !link.is_common && link.name != "main")
        .collect()
}

pub fn file(mod_rs: &Path, name: &str) -> Option<PathBuf> {
    path::child(mod_rs, name).filter(|found| !path::is_mod_or_lib(found))
}

pub fn folder(mod_rs: &Path, name: &str) -> Option<PathBuf> {
    path::child(mod_rs, name).filter(|found| path::is_mod_or_lib(found))
}

pub fn label(mod_rs: &Path, name: &str) -> String {
    match path::child(mod_rs, name) {
        Some(found) if path::is_mod_or_lib(&found) => format!("{name}/"),
        Some(_) => format!("{name}.rs"),
        None => name.to_string(),
    }
}

pub fn importer(mod_rs: &Path, importer: &Importer) -> String {
    match importer {
        Importer::Sibling(name) => label(mod_rs, name),
        Importer::Parent => path::rel(mod_rs),
        Importer::Elsewhere(file) => path::rel(file),
    }
}
