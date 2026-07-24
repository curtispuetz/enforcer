pub fn glob_module(tree: &syn::UseTree) -> Option<String> {
    match tree {
        syn::UseTree::Path(p) => match &*p.tree {
            syn::UseTree::Glob(_) => Some(p.ident.to_string()),
            inner => glob_module(inner),
        },
        _ => None,
    }
}
