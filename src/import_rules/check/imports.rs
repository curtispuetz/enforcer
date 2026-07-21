use syn::UseTree;

pub fn internal_use_paths(file: &syn::File) -> Vec<Vec<String>> {
    let mut ret = Vec::new();
    for item in &file.items {
        if let Some(tree) = use_tree(item) {
            for path in expand_use_tree(vec![], tree) {
                if is_internal(&path) {
                    ret.push(path);
                }
            }
        }
    }
    ret
}

fn use_tree(item: &syn::Item) -> Option<&UseTree> {
    match item {
        syn::Item::Use(u) => Some(&u.tree),
        _ => None,
    }
}

fn is_internal(path: &[String]) -> bool {
    matches!(
        path.first().map(String::as_str),
        Some("crate" | "super" | "self")
    )
}

fn expand_use_tree(prefix: Vec<String>, tree: &UseTree) -> Vec<Vec<String>> {
    match tree {
        UseTree::Path(p) => {
            expand_use_tree(pushed(prefix, p.ident.to_string()), &p.tree)
        }
        UseTree::Name(n) => vec![pushed(prefix, n.ident.to_string())],
        UseTree::Rename(r) => vec![pushed(prefix, r.ident.to_string())],
        UseTree::Glob(_) => vec![prefix],
        UseTree::Group(g) => g
            .items
            .iter()
            .flat_map(|item| expand_use_tree(prefix.clone(), item))
            .collect(),
    }
}

fn pushed(mut prefix: Vec<String>, segment: String) -> Vec<String> {
    prefix.push(segment);
    prefix
}
