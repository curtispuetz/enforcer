use super::{nca, t::CFn};

pub fn reason(cfn: &CFn, callers: &[Vec<String>]) -> Option<String> {
    if callers.is_empty() {
        return Some(format!("{}() is never called", cfn.name));
    }
    let ancestor = nca::common_prefix(callers);
    if ancestor.len() <= cfn.parent.len() {
        return None;
    }
    Some(_message(cfn, &ancestor))
}

fn _message(cfn: &CFn, ancestor: &[String]) -> String {
    let branch = ancestor.join("::");
    if ancestor.get(cfn.parent.len()).map(String::as_str) == Some("c") {
        return format!("{}() is only used inside its own `c` module ({branch})", cfn.name);
    }
    format!("{}() is only reached under {branch}; move it there", cfn.name)
}
