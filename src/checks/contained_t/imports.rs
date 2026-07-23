use std::collections::HashMap;

use syn::UseTree;

// not-obvious: Maps each name a `use` brings into scope to the full path it resolves
// to (e.g. `foo` -> `["crate", "x", "foo"]`), so a call's root segment can be traced
// back to whether it originates in our crate.
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

fn _expand(prefix: Vec<String>, tree: &UseTree) -> Vec<(String, Vec<String>)> {
    match tree {
        UseTree::Path(p) => _expand(_pushed(prefix, p.ident.to_string()), &p.tree),
        UseTree::Name(n) if n.ident == "self" => {
            let binding = prefix.last().cloned().unwrap_or_default();
            vec![(binding, prefix)]
        }
        UseTree::Name(n) => {
            let path = _pushed(prefix, n.ident.to_string());
            let binding = path.last().cloned().unwrap_or_default();
            vec![(binding, path)]
        }
        UseTree::Rename(r) => {
            let path = _pushed(prefix, r.ident.to_string());
            vec![(r.rename.to_string(), path)]
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

fn _pushed(mut prefix: Vec<String>, segment: String) -> Vec<String> {
    prefix.push(segment);
    prefix
}
