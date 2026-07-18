use std::collections::HashSet;
use std::path::Path;

use crate::files::rs_files;
use crate::source::parse_file;

pub fn collect_exported_macros(root: &Path, dir_name: &str) -> HashSet<String> {
    rs_files(root, dir_name)
        .iter()
        .flat_map(|path| exported_macros_in_file(path))
        .collect()
}

fn exported_macros_in_file(path: &Path) -> Vec<String> {
    parse_file(path)
        .items
        .iter()
        .filter_map(exported_macro_name)
        .collect()
}

fn exported_macro_name(item: &syn::Item) -> Option<String> {
    let syn::Item::Macro(m) = item else {
        return None;
    };
    let ident = m.ident.as_ref()?;
    is_macro_export(&m.attrs).then(|| ident.to_string())
}

fn is_macro_export(attrs: &[syn::Attribute]) -> bool {
    attrs.iter().any(|a| a.path().is_ident("macro_export"))
}

// A `#[macro_export]` macro lives at the crate root, so it is imported as `use crate::<name>;`.
pub fn is_exported_macro(use_path: &[String], exported_macros: &HashSet<String>) -> bool {
    use_path.len() == 2 && exported_macros.contains(&use_path[1])
}
