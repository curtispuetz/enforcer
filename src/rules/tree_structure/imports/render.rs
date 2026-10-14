use {
    super::t::{Fix, Violation},
    crate::rules::t::FileViolation,
    colored::Colorize,
};

pub fn violation(violation: Violation) -> FileViolation {
    let mut lines = Vec::new();
    for import in &violation.imports {
        lines.push(import.text.red().to_string());
        lines.push(format!("  {}", import.reason.dimmed()));
        if let Some(fix) = &import.fix {
            lines.push(format!("  {}", _fix_line(fix).dimmed()));
        }
    }
    FileViolation {
        path: violation.path,
        lines,
    }
}

fn _fix_line(fix: &Fix) -> String {
    match fix {
        Fix::Rewrite(text) => format!("suggested fix: {text}"),
        Fix::MoveToCommon => "suggested fix: move to a reachable common module".to_string(),
    }
}
