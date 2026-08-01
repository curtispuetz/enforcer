use std::path::PathBuf;

use super::{nca, t::CFn};

pub fn reason(cfn: &CFn, callers: &[Vec<String>]) -> Option<String> {
    if callers.is_empty() {
        return Some(format!("{}() is never called", cfn.name));
    }
    let ancestor = nca::common_prefix(callers);
    if ancestor == cfn.parent {
        return None;
    }
    Some(_message(cfn, &ancestor))
}

fn _message(cfn: &CFn, ancestor: &[String]) -> String {
    let branch = _branch_path(ancestor);
    if _above(ancestor, &cfn.parent) {
        return format!(
            "{}() is reached from outside its branch; move it up to {branch}",
            cfn.name
        );
    }
    if ancestor.len() < cfn.parent.len() {
        return format!(
            "{}() is reached from a different branch; move it to {branch}",
            cfn.name
        );
    }
    if ancestor.get(cfn.parent.len()).map(String::as_str) == Some("c") {
        return format!(
            "{}() is only used inside its own `c` module ({branch})",
            cfn.name
        );
    }
    format!(
        "{}() is only reached under {branch}; move it there",
        cfn.name
    )
}

fn _above(ancestor: &[String], parent: &[String]) -> bool {
    ancestor.len() < parent.len() && parent.starts_with(ancestor)
}

fn _branch_path(ancestor: &[String]) -> String {
    let rest = ancestor.iter().skip(1).cloned().collect::<Vec<_>>();
    let dir = std::iter::once("src".to_string())
        .chain(rest)
        .collect::<PathBuf>();
    if dir.is_dir() {
        return dir.display().to_string();
    }
    dir.with_extension("rs").display().to_string()
}
