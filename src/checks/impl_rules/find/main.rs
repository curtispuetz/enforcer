use std::collections::{HashMap, HashSet};
use std::path::Path;

use crate::c::path;

use super::{calls, impls};

pub fn violations(
    path: &Path,
    file: &syn::File,
    public_defs: &HashMap<String, Vec<String>>,
    crate_types: &HashSet<String>,
) -> Vec<String> {
    let rel = path::rel(path);
    let is_ext = path::under_dir(path, "ext_traits")
        || path::commons_file_kind(path) == Some("ext_traits");
    let is_t =
        path::under_dir(path, "t") || path::commons_file_kind(path) == Some("t");
    let mut out = impls::check(file, &rel, is_ext, public_defs, crate_types);
    if is_t {
        out.extend(calls::reaching_out(file));
    }
    out
}
