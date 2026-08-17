use {
    crate::rules::c::{calls, macros},
    syn::visit::{self, Visit},
};

pub struct CallsCollector {
    pub paths: Vec<Vec<String>>,
}

impl<'ast> Visit<'ast> for CallsCollector {
    fn visit_expr_call(&mut self, node: &'ast syn::ExprCall) {
        if let Some(segs) = calls::segments(&node.func) {
            self.paths.push(segs);
        }
        visit::visit_expr_call(self, node);
    }

    fn visit_macro(&mut self, node: &'ast syn::Macro) {
        for expr in macros::exprs(&node.tokens) {
            let mut nested = CallsCollector { paths: Vec::new() };
            nested.visit_expr(&expr);
            self.paths.append(&mut nested.paths);
        }
        visit::visit_macro(self, node);
    }
}
