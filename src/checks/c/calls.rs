pub fn is_function_like(name: &str) -> bool {
    matches!(name.chars().next(), Some(c) if c == '_' || c.is_ascii_lowercase())
}

pub fn segments(func: &syn::Expr) -> Option<Vec<String>> {
    let syn::Expr::Path(expr_path) = func else {
        return None;
    };
    if expr_path.qself.is_some() {
        return None;
    }
    let mut segments = Vec::new();
    for segment in &expr_path.path.segments {
        segments.push(segment.ident.to_string());
    }
    Some(segments)
}
