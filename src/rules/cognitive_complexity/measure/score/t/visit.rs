use {
    super::Scorer,
    syn::{
        Expr, ExprIf, ItemFn, Local,
        visit::{self, Visit},
    },
};

impl<'ast> Visit<'ast> for Scorer {
    fn visit_expr(&mut self, node: &'ast Expr) {
        if let Expr::Binary(bin) = node
            && self.logical_chain(bin)
        {
            return;
        }
        let saved = self.parent_logical;
        self.parent_logical = None;
        visit::visit_expr(self, node);
        self.parent_logical = saved;
    }

    fn visit_expr_if(&mut self, node: &'ast ExprIf) {
        self.score += 1 + self.nesting;
        self.if_chain(node);
    }

    fn visit_expr_match(&mut self, node: &'ast syn::ExprMatch) {
        self.scored_expr(&node.expr);
        self.nesting += 1;
        for arm in &node.arms {
            if let syn::Pat::Guard(guard) = &arm.pat {
                self.visit_expr(&guard.guard);
            }
            self.visit_expr(&arm.body);
        }
        self.nesting -= 1;
    }

    fn visit_expr_for_loop(&mut self, node: &'ast syn::ExprForLoop) {
        self.scored_expr(&node.expr);
        self.nested_block(&node.body);
    }

    fn visit_expr_while(&mut self, node: &'ast syn::ExprWhile) {
        self.scored_expr(&node.cond);
        self.nested_block(&node.body);
    }

    fn visit_expr_loop(&mut self, node: &'ast syn::ExprLoop) {
        self.score += 1 + self.nesting;
        self.nested_block(&node.body);
    }

    fn visit_expr_closure(&mut self, node: &'ast syn::ExprClosure) {
        self.nesting += 1;
        self.visit_expr(&node.body);
        self.nesting -= 1;
    }

    fn visit_expr_macro(&mut self, node: &'ast syn::ExprMacro) {
        self.branching_macro(&node.mac);
    }

    fn visit_local(&mut self, node: &'ast Local) {
        self.local_init(node);
    }

    fn visit_item_fn(&mut self, node: &'ast ItemFn) {
        self.nested_block(&node.block);
    }

    fn visit_expr_break(&mut self, node: &'ast syn::ExprBreak) {
        if node.label.is_some() {
            self.score += 1;
        }
        if let Some(expr) = &node.expr {
            self.visit_expr(expr);
        }
    }

    fn visit_expr_continue(&mut self, node: &'ast syn::ExprContinue) {
        if node.label.is_some() {
            self.score += 1;
        }
    }
}
