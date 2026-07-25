use std::path::Path;

use crate::rules::c::{ast, files};

pub fn of(path: &Path) -> Vec<String> {
    let mut issues = Vec::new();
    for item in files::ast_parse(path).items {
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
    _exposed_names(tree, &mut names, &mut glob);
    if glob {
        names.push("*".to_string());
    }
    names.join(", ")
}

fn _exposed_names(tree: &syn::UseTree, names: &mut Vec<String>, glob: &mut bool) {
    match tree {
        syn::UseTree::Name(n) => names.push(n.ident.to_string()),
        syn::UseTree::Rename(r) => names.push(r.rename.to_string()),
        syn::UseTree::Path(p) => _exposed_names(&p.tree, names, glob),
        syn::UseTree::Group(g) => {
            for item in &g.items {
                _exposed_names(item, names, glob);
            }
        }
        syn::UseTree::Glob(_) => *glob = true,
    }
}
