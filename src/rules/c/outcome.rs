use {
    super::path,
    crate::t::{ItemsViolation, Outcome},
    std::path::Path,
};

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
