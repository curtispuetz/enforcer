use {
    crate::{
        rules::{
            c::{outcome, parts, path},
            t::PartReport,
        },
        t::{ItemsViolation, Outcome},
    },
    std::path::Path,
};

pub fn part() -> PartReport {
    parts::from_files("mod-over-file", _check_file)
}

fn _check_file(path: &Path) -> Outcome<ItemsViolation> {
    if path::is_mod_or_lib(path) {
        return Outcome::Skipped;
    }
    outcome::of_items(path, _shadowed_folder(path))
}

fn _shadowed_folder(path: &Path) -> Vec<String> {
    match (path.parent(), path.file_stem().and_then(|s| s.to_str())) {
        (Some(parent), Some(stem)) if parent.join(stem).is_dir() => {
            vec![format!("shadows a sibling folder; move into {stem}/mod.rs")]
        }
        _ => Vec::new(),
    }
}
