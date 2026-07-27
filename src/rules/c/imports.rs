use std::collections::HashMap;

use syn::UseTree;

pub fn bindings(file: &syn::File) -> HashMap<String, Vec<String>> {
    let mut ret = HashMap::new();
    for item in &file.items {
        if let syn::Item::Use(u) = item {
            for (binding, path) in _expand(Vec::new(), &u.tree) {
                ret.insert(binding, path);
            }
        }
    }
    ret
}

pub fn bindings_and_items(
    file: &syn::File,
) -> (HashMap<String, Vec<String>>, Vec<String>) {
    (bindings(file), Vec::new())
}

fn _expand(mut prefix: Vec<String>, tree: &UseTree) -> Vec<(String, Vec<String>)> {
    match tree {
        UseTree::Path(p) => {
            prefix.push(p.ident.to_string());
            _expand(prefix, &p.tree)
        }
        UseTree::Name(n) => {
            if n.ident != "self" {
                prefix.push(n.ident.to_string());
            }
            let binding = prefix.last().cloned().unwrap_or_default();
            vec![(binding, prefix)]
        }
        UseTree::Rename(r) => {
            prefix.push(r.ident.to_string());
            vec![(r.rename.to_string(), prefix)]
        }
        UseTree::Glob(_) => Vec::new(),
        UseTree::Group(g) => {
            let mut ret = Vec::new();
            for item in &g.items {
                ret.extend(_expand(prefix.clone(), item));
            }
            ret
        }
    }
}
