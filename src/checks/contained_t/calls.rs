use syn::visit::{self, Visit};

// not-obvious: Collects the path of every function-call expression (e.g. the
// `crate::x::foo` of `crate::x::foo(..)`). Method calls (`x.foo()`) are a different
// syn node and are intentionally ignored — they are not free-function calls.
pub fn paths(file: &syn::File) -> Vec<Vec<String>> {
    let mut collector = Collector { paths: Vec::new() };
    collector.visit_file(file);
    collector.paths
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
