use {
    super::{Logical, Scorer},
    crate::rules::cognitive_complexity::measure::score::cnst::BRANCHING_MACROS,
    syn::{BinOp, ExprBinary, Local, Macro, visit::Visit},
};

impl Scorer {
    pub fn logical_chain(&mut self, bin: &ExprBinary) -> bool {
        let Some(op) = _logical(&bin.op) else {
            return false;
        };
        if self.parent_logical != Some(op) {
            self.score += 1;
        }
        let saved = self.parent_logical;
        self.parent_logical = Some(op);
        self.visit_expr(&bin.left);
        self.parent_logical = Some(op);
        self.visit_expr(&bin.right);
        self.parent_logical = saved;
        true
    }

    pub fn branching_macro(&mut self, mac: &Macro) {
        if _is_branching(mac) {
            self.score += 1 + self.nesting;
        }
    }

    pub fn local_init(&mut self, node: &Local) {
        let Some(init) = &node.init else {
            return;
        };
        self.visit_expr(&init.expr);
        let Some((_, diverge)) = &init.diverge else {
            return;
        };
        self.score += 1 + self.nesting;
        self.nesting += 1;
        self.visit_expr(diverge);
        self.nesting -= 1;
    }
}

fn _is_branching(mac: &Macro) -> bool {
    let Some(name) = mac.path.segments.last() else {
        return false;
    };
    BRANCHING_MACROS.contains(&name.ident.to_string().as_str())
}

fn _logical(op: &BinOp) -> Option<Logical> {
    match op {
        BinOp::And(_) => Some(Logical::And),
        BinOp::Or(_) => Some(Logical::Or),
        _ => None,
    }
}
