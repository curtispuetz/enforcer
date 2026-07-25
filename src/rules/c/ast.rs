pub fn is_public(vis: &syn::Visibility) -> bool {
    !matches!(vis, syn::Visibility::Inherited)
}
