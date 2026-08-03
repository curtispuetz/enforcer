use {
    super::scan,
    crate::{
        rules::t::{FileViolation, PartReport},
        s::FILE_CONFIG,
        t::{ItemsViolation, Outcome},
    },
    colored::Colorize,
    std::path::Path,
};

pub fn from_files(
    name: &'static str,
    failures_header: &'static str,
    rule: impl FnMut(&Path) -> Outcome<ItemsViolation>,
) -> PartReport {
    let res = scan::src_files(rule);
    PartReport {
        name,
        failures_header,
        unit: "files",
        passed: res.passed,
        violations: res.violations.into_iter().map(FileViolation::new).collect(),
    }
}

pub fn print(name: &str, parts: Vec<PartReport>) -> bool {
    let any_failed = parts.iter().any(|p| !p.violations.is_empty());
    if !any_failed && !FILE_CONFIG.debug {
        return false;
    }
    println!("{}", format!("{name} report:").bold().cyan());
    println!();
    for part in &parts {
        if FILE_CONFIG.debug || !part.violations.is_empty() {
            _status_line(part);
        }
    }
    if !any_failed {
        return false;
    }
    for part in &parts {
        _failures(part);
    }
    true
}

fn _status_line(part: &PartReport) {
    let failed = part.violations.len();
    let label = format!("{:<16}", part.name);
    if failed == 0 {
        let mark = "\u{2714}".green().bold();
        let msg = format!("{} {} passed", part.passed, part.unit).green();
        println!("  {mark} {} {msg}", label.bold());
    } else {
        let mark = "\u{2718}".red().bold();
        let total = part.passed + failed;
        let msg = format!("{failed} of {total} {} failed", part.unit)
            .red()
            .bold();
        println!("  {mark} {} {msg}", label.bold());
    }
}

fn _failures(part: &PartReport) {
    if part.violations.is_empty() {
        return;
    }
    println!("\n{}\n", part.failures_header.bold());
    for violation in &part.violations {
        println!("  {}", violation.path.bold());
        for line in &violation.lines {
            println!("    {line}");
        }
    }
}
