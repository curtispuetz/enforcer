use std::{fs, path::Path};

pub fn parse_file(path: &Path) -> syn::File {
    let source = fs::read_to_string(path).expect("failed to read file");
    syn::parse_file(&source).expect("failed to parse file")
}
