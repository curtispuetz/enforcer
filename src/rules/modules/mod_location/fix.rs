use {
    crate::rules::c::{files, path},
    std::path::Path,
};

const BIN_ROOT_FIX: &str =
    "move the mods into src/lib.rs and reach them from main.rs through the crate name";

pub fn of(file: &Path) -> Option<String> {
    let name = file.file_name()?.to_str()?;
    let stem = file.file_stem()?.to_str()?;
    if path::rel(file) == "src/main.rs" {
        return Some(BIN_ROOT_FIX.to_string());
    }
    if _is_crate_root(file) {
        return None;
    }
    if _only_mods_and_uses(file) {
        return Some(format!("move {name} to {stem}/mod.rs"));
    }
    Some(format!(
        "move the mods into {stem}/mod.rs, and the rest of {name} into {stem}/main.rs, \
        re-exported with pub use main::…"
    ))
}

fn _is_crate_root(file: &Path) -> bool {
    let dir = file.parent().map(path::rel);
    matches!(dir.as_deref(), Some("src/bin" | "tests"))
}

fn _only_mods_and_uses(file: &Path) -> bool {
    files::ast_parse(file)
        .items
        .iter()
        .all(|item| matches!(item, syn::Item::Mod(_) | syn::Item::Use(_)))
}
