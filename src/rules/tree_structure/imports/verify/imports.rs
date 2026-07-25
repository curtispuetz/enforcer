use syn::UseTree;

pub fn internal_use_paths(file: &syn::File) -> Vec<Vec<String>> {
    let mut ret = Vec::new();
    for item in &file.items {
        if let Some(tree) = _use_tree(item) {
            _add_tree_use_paths(tree, &mut ret);
        }
    }
    ret
}

fn _add_tree_use_paths(tree: &UseTree, ret: &mut Vec<Vec<String>>) {
    for path in _expand_use_tree(vec![], tree) {
        if _is_internal(&path) {
            ret.push(path);
        }
    }
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

fn _expand_use_tree(mut prefix: Vec<String>, tree: &UseTree) -> Vec<Vec<String>> {
    match tree {
        UseTree::Path(p) => {
            prefix.push(p.ident.to_string());
            _expand_use_tree(prefix, &p.tree)
        }
        UseTree::Name(n) => {
            prefix.push(n.ident.to_string());
            vec![prefix]
        }
        UseTree::Rename(r) => {
            prefix.push(r.ident.to_string());
            vec![prefix]
        }
        UseTree::Glob(_) => vec![prefix],
        UseTree::Group(g) => g
            .items
            .iter()
            .flat_map(|item| _expand_use_tree(prefix.clone(), item))
            .collect(),
    }
}
