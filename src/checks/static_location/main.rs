use std::path::Path;

use crate::c::{ast, files, path, scan};

use super::{report::report, t::violation::Violation};

pub fn run() -> bool {
    let (passed, violations) = scan::src_files(_check_file);
    report(passed, violations)
}

fn _check_file(path: &Path) -> Option<Violation> {
    if path::under_dir(path, "s") {
        return None;
    }
    let items = _misplaced_statics(path);
    if items.is_empty() {
        None
    } else {
        Some(Violation {
            path: path::rel(path),
            items,
        })
    }
}

fn _misplaced_statics(path: &Path) -> Vec<String> {
    let mut items = Vec::new();
    for item in files::parse(path).items {
        if let syn::Item::Static(i) = &item
            && ast::is_public(&i.vis)
        {
            items.push(format!("static {}", i.ident));
        }
    }
    items
}
