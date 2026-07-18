use syn::UseTree;

pub fn crate_use_paths(file: &syn::File) -> Vec<Vec<String>> {
    file.items
        .iter()
        .filter_map(use_tree)
        .flat_map(|tree| expand_use_tree(vec![], tree))
        .filter(|path| starts_at_crate(path))
        .collect()
}

fn use_tree(item: &syn::Item) -> Option<&UseTree> {
    match item {
        syn::Item::Use(u) => Some(&u.tree),
        _ => None,
    }
}

fn starts_at_crate(path: &[String]) -> bool {
    path.first().map(String::as_str) == Some("crate")
}

fn expand_use_tree(prefix: Vec<String>, tree: &UseTree) -> Vec<Vec<String>> {
    match tree {
        UseTree::Path(p) => expand_use_tree(pushed(prefix, p.ident.to_string()), &p.tree),
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
