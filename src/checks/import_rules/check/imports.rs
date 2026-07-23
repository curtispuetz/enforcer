use syn::UseTree;

pub fn internal_use_paths(file: &syn::File) -> Vec<Vec<String>> {
    let mut ret = Vec::new();
    for item in &file.items {
        if let Some(tree) = _use_tree(item) {
            for path in _expand_use_tree(vec![], tree) {
                if _is_internal(&path) {
                    ret.push(path);
                }
            }
        }
    }
    ret
}

fn _use_tree(item: &syn::Item) -> Option<&UseTree> {
    match item {
        syn::Item::Use(u) => Some(&u.tree),
        _ => None,
    }
}

fn _is_internal(path: &[String]) -> bool {
    matches!(path.first().map(String::as_str), Some("crate" | "super"))
}

fn _expand_use_tree(prefix: Vec<String>, tree: &UseTree) -> Vec<Vec<String>> {
    match tree {
        UseTree::Path(p) => {
            _expand_use_tree(_pushed(prefix, p.ident.to_string()), &p.tree)
        }
        UseTree::Name(n) => vec![_pushed(prefix, n.ident.to_string())],
        UseTree::Rename(r) => vec![_pushed(prefix, r.ident.to_string())],
        UseTree::Glob(_) => vec![prefix],
        UseTree::Group(g) => g
            .items
            .iter()
            .flat_map(|item| _expand_use_tree(prefix.clone(), item))
            .collect(),
    }
}

fn _pushed(mut prefix: Vec<String>, segment: String) -> Vec<String> {
    prefix.push(segment);
    prefix
}
