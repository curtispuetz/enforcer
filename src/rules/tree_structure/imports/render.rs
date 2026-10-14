use {
    super::t::{Fix, Violation},
    crate::rules::{c::suggestions, t::FileViolation},
    colored::Colorize,
};

pub fn violation(violation: Violation) -> FileViolation {
    let mut lines = Vec::new();
    for import in &violation.imports {
        lines.push(import.text.red().to_string());
        lines.push(format!("  {}", import.reason.dimmed()));
        let fixes: Vec<String> = import.fix.iter().map(_fix_text).collect();
        lines.extend(suggestions::lines(&fixes).iter().map(|l| format!("  {l}")));
    }
    FileViolation {
        path: violation.path,
        lines,
    }
}

fn _fix_text(fix: &Fix) -> String {
    match fix {
        Fix::Rewrite(text) => text.clone(),
        Fix::MoveToCommon => "move to a reachable common module".to_string(),
    }
}
