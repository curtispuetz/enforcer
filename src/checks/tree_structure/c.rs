use colored::Colorize;

use crate::t::ItemsViolation;

use super::t::FileViolation;

pub fn file_violation(violation: ItemsViolation) -> FileViolation {
    FileViolation {
        path: violation.path,
        lines: violation.items.iter().map(|i| i.red().to_string()).collect(),
    }
}
