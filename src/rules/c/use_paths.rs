use {
    super::test_only,
    syn::{Ident, UseTree},
};

pub fn internal(file: &syn::File) -> Vec<Vec<String>> {
    _internal(file, |_| true)
}

pub fn production(file: &syn::File) -> Vec<Vec<String>> {
    _internal(file, |u| !test_only::attrs(&u.attrs))
}

fn _internal(
    file: &syn::File,
    keep: impl Fn(&syn::ItemUse) -> bool,
) -> Vec<Vec<String>> {
    let mut ret = Vec::new();
    for item in &file.items {
        if let Some(tree) = _use_tree(item, &keep) {
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

fn _use_tree<'a>(
    item: &'a syn::Item,
    keep: &impl Fn(&syn::ItemUse) -> bool,
) -> Option<&'a UseTree> {
    match item {
        syn::Item::Use(u) if keep(u) => Some(&u.tree),
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
        UseTree::Name(n) => _leaf(prefix, &n.ident),
        UseTree::Rename(r) => _leaf(prefix, &r.ident),
        UseTree::Glob(_) => vec![prefix],
        UseTree::Group(g) => g
            .items
            .iter()
            .flat_map(|item| _expand_use_tree(prefix.clone(), item))
            .collect(),
    }
}

fn _leaf(mut prefix: Vec<String>, ident: &Ident) -> Vec<Vec<String>> {
    prefix.push(ident.to_string());
    vec![prefix]
}
