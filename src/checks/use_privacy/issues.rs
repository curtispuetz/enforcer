use std::path::Path;

use crate::c::{ast, files};

pub fn of(path: &Path) -> Vec<String> {
    let mut issues = Vec::new();
    for item in files::parse(path).items {
        if let syn::Item::Use(u) = &item
            && ast::is_public(&u.vis)
        {
            issues.push(format!("public use of {}", _exposed(&u.tree)));
        }
    }
    issues
}

fn _exposed(tree: &syn::UseTree) -> String {
    let mut names = Vec::new();
    let mut glob = false;
    ast::exposed_names(tree, &mut names, &mut glob);
    if glob {
        names.push("*".to_string());
    }
    names.join(", ")
}
