use std::{fs, path::Path};

use crate::{
    rules::{
        c::{files, path},
        private_fn_naming::{c::names, t::Misnamed},
    },
    s::EXISTING_SRC_DIRS,
};

use super::{apply, report, spots};

pub fn fix() {
    let mut fixed = Vec::new();
    for dir_name in EXISTING_SRC_DIRS.iter() {
        for file_path in files::rs(dir_name) {
            if let Some(line) = _fix_file(&file_path) {
                fixed.push(line);
            }
        }
    }
    report::print(fixed);
}

fn _fix_file(file_path: &Path) -> Option<String> {
    let file = files::ast_parse(file_path);
    let misnamed = names::misnamed(&file);
    if misnamed.free.is_empty() && misnamed.methods.is_empty() {
        return None;
    }
    _rewrite(file_path, &file, &misnamed)
}

fn _rewrite(file_path: &Path, file: &syn::File, misnamed: &Misnamed) -> Option<String> {
    let (found, renamed) = spots::of_names(file, misnamed);
    if found.is_empty() {
        return None;
    }
    let source = fs::read_to_string(file_path).expect("failed to read file");
    fs::write(file_path, apply::underscores(&source, &found))
        .expect("failed to write file");
    Some(format!("{} ({})", path::rel(file_path), renamed.join(", ")))
}
