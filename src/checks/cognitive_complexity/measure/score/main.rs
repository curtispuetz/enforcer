use syn::visit::Visit;
use syn::{Block, Expr, ExprIf};

use super::t::Scorer;

pub fn of(block: &Block) -> usize {
    let mut scorer = Scorer {
        score: 0,
        nesting: 0,
        parent_logical: None,
    };
    scorer.visit_block(block);
    scorer.score
}

impl Scorer {
    pub fn nested_block(&mut self, block: &Block) {
        self.nesting += 1;
        self.visit_block(block);
        self.nesting -= 1;
    }

    pub fn if_chain(&mut self, node: &ExprIf) {
        self.visit_expr(&node.cond);
        self.nested_block(&node.then_branch);
        self.else_chain(&node.else_branch);
    }

    pub fn else_chain(&mut self, branch: &Option<(syn::token::Else, Box<Expr>)>) {
        let Some((_, else_expr)) = branch else {
            return;
        };
        self.score += 1;
        match &**else_expr {
            Expr::If(elif) => self.if_chain(elif),
            other => {
                self.nesting += 1;
                self.visit_expr(other);
                self.nesting -= 1;
            }
        }
    }
}
