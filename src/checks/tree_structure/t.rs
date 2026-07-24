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
