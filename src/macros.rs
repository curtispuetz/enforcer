use std::{collections::HashSet, path::Path};

use crate::c::files;

pub fn collect_exported(dir_name: &str) -> HashSet<String> {
    files::rs(dir_name)
        .iter()
        .flat_map(|path| _exported_macros_in_file(path))
        .collect()
}

fn _exported_macros_in_file(path: &Path) -> Vec<String> {
    files::parse(path)
        .items
        .iter()
        .filter_map(_exported_macro_name)
        .collect()
}

fn _exported_macro_name(item: &syn::Item) -> Option<String> {
    let syn::Item::Macro(m) = item else {
        return None;
    };
    let ident = m.ident.as_ref()?;
    _is_macro_export(&m.attrs).then(|| ident.to_string())
}

fn _is_macro_export(attrs: &[syn::Attribute]) -> bool {
    attrs.iter().any(|a| a.path().is_ident("macro_export"))
}
