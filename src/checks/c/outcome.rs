use std::path::Path;

use crate::t::{ItemsViolation, Outcome};

use super::path;

pub fn of_items(file: &Path, items: Vec<String>) -> Outcome<ItemsViolation> {
    if items.is_empty() {
        Outcome::Passed
    } else {
        Outcome::Failed(ItemsViolation {
            path: path::rel(file),
            items,
        })
    }
}
