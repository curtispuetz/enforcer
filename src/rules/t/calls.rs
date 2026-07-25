use crate::rules::c::calls;
use syn::visit::{self, Visit};

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
}
