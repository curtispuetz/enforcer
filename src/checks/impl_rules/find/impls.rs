use std::collections::{HashMap, HashSet};

use super::message;

pub fn check(
    file: &syn::File,
    rel: &str,
    is_ext: bool,
    public_defs: &HashMap<String, Vec<String>>,
    crate_types: &HashSet<String>,
) -> Vec<String> {
    let mut out = Vec::new();
    for item in &file.items {
        let syn::Item::Impl(imp) = item else {
            continue;
        };
        let Some(name) = _self_ident(imp) else {
            continue;
        };
        if imp.trait_.is_some() && !is_ext && !crate_types.contains(&name) {
            out.push(message::foreign_trait_impl(&name));
        } else if !is_ext
            && let Some(msg) = _misplaced(&name, rel, public_defs)
        {
            out.push(msg);
        }
    }
    out
}

fn _misplaced(
    name: &str,
    rel: &str,
    public_defs: &HashMap<String, Vec<String>>,
) -> Option<String> {
    let files = public_defs.get(name)?;
    if files.iter().any(|f| _same_t_module(f, rel)) {
        return None;
    }
    // A type in a `t/` directory admits sibling-module impls; one in a single-file
    // `t.rs` module has no siblings, so the impl simply belongs in a t module.
    if files.iter().any(|f| _t_module(f) != f.as_str()) {
        Some(message::misplaced_impl(name))
    } else {
        Some(message::not_in_t_module(name))
    }
}

fn _same_t_module(a: &str, b: &str) -> bool {
    _t_module(a) == _t_module(b)
}

// The t commons module containing a file: a `t/` directory admits any file under
// it (so an impl may spread across sibling files), whereas a single-file `t.rs`
// module is only itself (its directory siblings are not part of the module).
fn _t_module(p: &str) -> &str {
    match p.find("/t/") {
        Some(i) => &p[..i + 2],
        None => p,
    }
}

fn _self_ident(imp: &syn::ItemImpl) -> Option<String> {
    let syn::Type::Path(p) = &*imp.self_ty else {
        return None;
    };
    p.path.segments.last().map(|s| s.ident.to_string())
}
