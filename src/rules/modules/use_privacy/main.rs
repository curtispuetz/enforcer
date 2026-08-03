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

pub fn part() -> PartReport {
    parts::from_files(
        "use-privacy",
        "The following file(s) have a `pub use` outside mod.rs or lib.rs:",
        _check_file,
    )
}

fn _check_file(path: &Path) -> Outcome<ItemsViolation> {
    if path::is_mod_or_lib(path) {
        return Outcome::Skipped;
    }
    outcome::of_items(path, issues::of(path))
}
