use std::path::PathBuf;

use crate::t::ItemsViolation;
use colored::Colorize;

pub struct SurfaceItem {
    pub kind: &'static str,
    pub name: String,
    pub file: PathBuf,
    pub module: Vec<String>,
}

pub struct PartReport {
    pub name: &'static str,
    pub unit: &'static str,
    pub passed: usize,
    pub violations: Vec<FileViolation>,
}

pub struct FileViolation {
    pub path: String,
    pub lines: Vec<String>,
}

impl FileViolation {
    pub fn new(violation: ItemsViolation) -> Self {
        FileViolation {
            path: violation.path,
            lines: violation
                .items
                .iter()
                .map(|i| i.red().to_string())
                .collect(),
        }
    }
}
