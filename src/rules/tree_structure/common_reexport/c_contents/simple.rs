use crate::rules::c::ast;

pub fn violations(file: &syn::File) -> Vec<String> {
    let mut issues = Vec::new();
    for item in &file.items {
        match item {
            syn::Item::Use(_) => {
                issues.push("unexpected `use` statement".to_string());
            }
            syn::Item::Mod(m) if !ast::is_public(&m.vis) => {
                issues.push(format!("module `{}` is not public", m.ident));
            }
            _ => {}
        }
    }
    issues
}
