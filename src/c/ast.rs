pub fn is_public(vis: &syn::Visibility) -> bool {
    !matches!(vis, syn::Visibility::Inherited)
}

pub fn self_base_name(ty: &syn::Type) -> Option<String> {
    match ty {
        syn::Type::Path(p) => p.path.segments.last().map(|s| s.ident.to_string()),
        syn::Type::Reference(r) => self_base_name(&r.elem),
        _ => None,
    }
}

pub fn glob_module(tree: &syn::UseTree) -> Option<String> {
    match tree {
        syn::UseTree::Path(p) => match &*p.tree {
            syn::UseTree::Glob(_) => Some(p.ident.to_string()),
            inner => glob_module(inner),
        },
        _ => None,
    }
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
