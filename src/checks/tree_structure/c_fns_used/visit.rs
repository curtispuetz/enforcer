use syn::visit::{self, Visit};

use crate::c::calls;

pub fn scoped(file: &syn::File) -> Vec<(Option<String>, Vec<String>)> {
    let mut collector = Collector {
        scope: Vec::new(),
        out: Vec::new(),
    };
    collector.visit_file(file);
    collector.out
}

struct Collector {
    scope: Vec<String>,
    out: Vec<(Option<String>, Vec<String>)>,
}

impl<'ast> Visit<'ast> for Collector {
    fn visit_item_fn(&mut self, node: &'ast syn::ItemFn) {
        self.scope.push(node.sig.ident.to_string());
        visit::visit_item_fn(self, node);
        self.scope.pop();
    }

    fn visit_impl_item_fn(&mut self, node: &'ast syn::ImplItemFn) {
        self.scope.push(node.sig.ident.to_string());
        visit::visit_impl_item_fn(self, node);
        self.scope.pop();
    }

    fn visit_expr_call(&mut self, node: &'ast syn::ExprCall) {
        if let Some(segs) = calls::segments(&node.func) {
            self.out.push((self.scope.last().cloned(), segs));
        }
        visit::visit_expr_call(self, node);
    }
}
