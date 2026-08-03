use {
    super::{contents, home},
    crate::{
        rules::{
            c::{files, parts, path},
            t::PartReport,
        },
        t::{ItemsViolation, Outcome},
    },
    std::{collections::HashMap, path::Path},
};

pub fn part() -> PartReport {
    let disallowed = contents::disallowed();
    parts::from_files(
        "common-items",
        "The following file(s) define an item outside its common module home, or expose an item a common module may not expose:",
        |path| _check_file(path, &disallowed),
    )
}

fn _check_file(
    path: &Path,
    disallowed: &HashMap<String, Vec<String>>,
) -> Outcome<ItemsViolation> {
    let rel = path::rel(path);
    let file = files::ast_parse(path);
    let mut items = home::misplaced(path, &file);
    items.extend(disallowed.get(&rel).cloned().unwrap_or_default());
    if items.is_empty() {
        Outcome::Passed
    } else {
        Outcome::Failed(ItemsViolation { path: rel, items })
    }
}
