use std::collections::HashSet;
use std::{fs, path::Path};

use crate::check::rs_files;

/// Scan a directory for macros defined with `#[macro_export]` and add their
/// names to `out`. Such macros live at the crate root and are therefore
/// imported as `use crate::<name>;`.
pub fn collect_exported_macros(root: &Path, dir_name: &str, out: &mut HashSet<String>) {
    for path in rs_files(root, dir_name) {
        let source = fs::read_to_string(&path).expect("failed to read file");
        let file = syn::parse_file(&source).expect("failed to parse file");
        for item in &file.items {
            if let syn::Item::Macro(m) = item {
                if let Some(ident) = &m.ident {
                    if m.attrs.iter().any(|a| a.path().is_ident("macro_export")) {
                        out.insert(ident.to_string());
                    }
                }
            }
        }
    }
}

/// A `#[macro_export]` macro lives at the crate root, so it is imported as
/// `use crate::<name>;`. Return true if `use_path` names such a macro.
pub fn is_exported_macro(use_path: &[String], exported_macros: &HashSet<String>) -> bool {
    use_path.len() == 2 && exported_macros.contains(&use_path[1])
}
