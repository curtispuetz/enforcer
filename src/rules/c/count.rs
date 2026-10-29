use {super::files, std::path::Path};

pub fn mods(path: &Path) -> usize {
    counted(path).len()
}

pub fn counted(path: &Path) -> Vec<String> {
    files::ast_parse(path)
        .items
        .iter()
        .filter_map(|item| match item {
            syn::Item::Mod(m) if !_is_cfg_gated(&m.attrs) => Some(m.ident.to_string()),
            _ => None,
        })
        .collect()
}

fn _is_cfg_gated(attrs: &[syn::Attribute]) -> bool {
    attrs.iter().any(|attr| attr.path().is_ident("cfg"))
}
