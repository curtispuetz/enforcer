use {super::ast, crate::rules::t::Binding, std::collections::HashMap, syn::UseTree};

pub fn bindings(file: &syn::File) -> HashMap<String, Vec<String>> {
    let mut ret = HashMap::new();
    for item in &file.items {
        if let syn::Item::Use(u) = item {
            ret.extend(_expand(Vec::new(), &u.tree, &mut Vec::new()));
        }
    }
    ret
}

pub fn bindings_and_items(
    file: &syn::File,
) -> (HashMap<String, Vec<String>>, Vec<String>) {
    (bindings(file), Vec::new())
}

pub fn reexports(file: &syn::File) -> (Vec<Binding>, Vec<Vec<String>>) {
    let mut named = Vec::new();
    let mut globs = Vec::new();
    for item in &file.items {
        if let syn::Item::Use(u) = item
            && ast::is_public(&u.vis)
        {
            named.extend(_expand(Vec::new(), &u.tree, &mut globs));
        }
    }
    (named, globs)
}

fn _expand(
    mut prefix: Vec<String>,
    tree: &UseTree,
    globs: &mut Vec<Vec<String>>,
) -> Vec<Binding> {
    match tree {
        UseTree::Path(p) => {
            prefix.push(p.ident.to_string());
            _expand(prefix, &p.tree, globs)
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
        UseTree::Glob(_) => {
            globs.push(prefix);
            Vec::new()
        }
        UseTree::Group(g) => {
            let mut ret = Vec::new();
            for item in &g.items {
                ret.extend(_expand(prefix.clone(), item, globs));
            }
            ret
        }
    }
}
