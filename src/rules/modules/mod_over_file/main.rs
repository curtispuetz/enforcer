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
    parts::with_fixes(
        "mod-over-file",
        "The following file(s) shadow a sibling folder and should become that folder's mod.rs:",
        _check_file,
        |path, _| _fix(path).into_iter().collect(),
    )
}

fn _check_file(path: &Path) -> Outcome<ItemsViolation> {
    if path::is_mod_or_lib(path) {
        return Outcome::Skipped;
    }
    outcome::of_items(path, _shadowed_folder(path))
}

fn _shadowed_folder(path: &Path) -> Vec<String> {
    match (path.parent(), path.file_stem()) {
        (Some(parent), Some(stem)) if parent.join(stem).is_dir() => {
            vec!["shadows a sibling folder".to_string()]
        }
        _ => Vec::new(),
    }
}

fn _fix(path: &Path) -> Option<String> {
    let stem = path.file_stem()?.to_str()?;
    let verb = if path.with_extension("").join("mod.rs").is_file() {
        "merge it into"
    } else {
        "move it to"
    };
    Some(format!("{verb} {stem}/mod.rs"))
}
