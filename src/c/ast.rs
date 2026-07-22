pub fn is_public(vis: &syn::Visibility) -> bool {
    !matches!(vis, syn::Visibility::Inherited)
}

pub fn exposed_names(tree: &syn::UseTree, names: &mut Vec<String>, glob: &mut bool) {
    match tree {
        syn::UseTree::Name(n) => names.push(n.ident.to_string()),
        syn::UseTree::Rename(r) => names.push(r.rename.to_string()),
        syn::UseTree::Path(p) => exposed_names(&p.tree, names, glob),
        syn::UseTree::Group(g) => {
            for item in &g.items {
                exposed_names(item, names, glob);
            }
        }
        syn::UseTree::Glob(_) => *glob = true,
    }
}
