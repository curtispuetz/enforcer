use std::collections::HashMap;

use syn::visit::{self, Visit};

use super::{imports, message};

pub fn reaching_out(file: &syn::File) -> Vec<String> {
    let imported = imports::bindings(file);
    let mut collector = Collector { paths: Vec::new() };
    collector.visit_file(file);
    let mut out = Vec::new();
    for segments in collector.paths {
        if _reaches_crate(&segments, &imported) {
            out.push(message::free_fn(&segments));
        }
    }
    out
}

struct Collector {
    paths: Vec<Vec<String>>,
}

impl<'ast> Visit<'ast> for Collector {
    fn visit_expr_call(&mut self, node: &'ast syn::ExprCall) {
        if let Some(segments) = _segments(&node.func) {
            self.paths.push(segments);
        }
        visit::visit_expr_call(self, node);
    }
}

fn _segments(func: &syn::Expr) -> Option<Vec<String>> {
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

fn _reaches_crate(
    segments: &[String],
    imported: &HashMap<String, Vec<String>>,
) -> bool {
    let Some(name) = segments.last() else {
        return false;
    };
    if !_is_lower(name) {
        return false;
    }
    if segments.len() == 1 {
        return imported.get(name).is_some_and(|p| _rooted_at_crate(p));
    }
    _root_reaches_crate(&segments[0], imported)
}

fn _root_reaches_crate(
    root: &str,
    imported: &HashMap<String, Vec<String>>,
) -> bool {
    root == "crate" || imported.get(root).is_some_and(|p| _rooted_at_crate(p))
}

fn _rooted_at_crate(path: &[String]) -> bool {
    path.first().map(String::as_str) == Some("crate")
}

fn _is_lower(s: &str) -> bool {
    matches!(s.chars().next(), Some(c) if c == '_' || c.is_ascii_lowercase())
}
