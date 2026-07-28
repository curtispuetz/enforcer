use syn::{
    Block, Expr, Pat, Stmt, Type,
    visit::{self, Visit},
};

pub fn count(skeleton: &Block) -> usize {
    let mut counter = Counter::default();
    counter.visit_block(skeleton);
    counter.total
}

#[derive(Default)]
struct Counter {
    total: usize,
}

impl<'ast> Visit<'ast> for Counter {
    fn visit_stmt(&mut self, stmt: &'ast Stmt) {
        self.total += 1;
        visit::visit_stmt(self, stmt);
    }

    fn visit_expr(&mut self, expr: &'ast Expr) {
        self.total += 1;
        visit::visit_expr(self, expr);
    }

    fn visit_pat(&mut self, pat: &'ast Pat) {
        self.total += 1;
        visit::visit_pat(self, pat);
    }

    fn visit_type(&mut self, ty: &'ast Type) {
        self.total += 1;
        visit::visit_type(self, ty);
    }
}
