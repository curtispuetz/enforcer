use {
    super::issues,
    crate::{
        rules::{
            c::{outcome, parts, path},
            t::PartReport,
        },
        t::{ItemsViolation, Outcome},
    },
    std::path::Path,
};

const PARENTS: [&str; 3] = ["mod.rs", "lib.rs", "main.rs"];

pub fn part() -> PartReport {
    parts::with_fixes(
        "use-privacy",
        "The following file(s) have a `pub use` outside mod.rs or lib.rs:",
        _check_file,
        |path, violation| vec![_fix(path, violation.items.len())],
    )
}

fn _check_file(path: &Path) -> Outcome<ItemsViolation> {
    if path::is_mod_or_lib(path) {
        return Outcome::Skipped;
    }
    outcome::of_items(path, issues::of(path))
}

fn _fix(path: &Path, count: usize) -> String {
    let (it, uses) = if count == 1 {
        ("it", "a private use")
    } else {
        ("them", "private uses")
    };
    match _parent(path) {
        Some(parent) => {
            format!(
                "make {it} {uses}, and re-export {it} from {parent} if other modules need {it}"
            )
        }
        None => format!("make {it} {uses}"),
    }
}

fn _parent(path: &Path) -> Option<&'static str> {
    PARENTS.into_iter().find(|name| {
        path.file_name() != Some(name.as_ref()) && path.with_file_name(name).is_file()
    })
}
