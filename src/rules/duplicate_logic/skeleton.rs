use {
    super::{
        nodes,
        t::{HoleKind, Leaf, Shape},
    },
    proc_macro2::{Ident, Span},
    syn::{
        Block, ExprMethodCall, Lit, LitStr, Member, TypePath,
        visit_mut::{self, VisitMut},
    },
};

// not-obvious: the skeleton is the canonical block with every leaf that could
// become a helper parameter blanked out, so two fragments that differ only in
// those leaves hash to the same key
pub fn blank(canonical: &Block) -> Shape {
    let mut block = canonical.clone();
    let mut blanker = Blanker::default();
    blanker.visit_block_mut(&mut block);
    let counted = nodes::count(&block);
    Shape {
        block,
        leaves: blanker.leaves,
        nodes: counted,
    }
}

#[derive(Default)]
struct Blanker {
    leaves: Vec<Leaf>,
}

impl Blanker {
    fn _hole(&mut self, ident: &mut Ident, kind: HoleKind) {
        self.leaves.push(Leaf {
            kind,
            text: ident.to_string(),
        });
        *ident = Ident::new("__hole", Span::call_site());
    }
}

impl VisitMut for Blanker {
    fn visit_member_mut(&mut self, member: &mut Member) {
        if let Member::Named(ident) = member {
            self._hole(ident, HoleKind::Field);
        }
    }

    fn visit_expr_method_call_mut(&mut self, call: &mut ExprMethodCall) {
        self._hole(&mut call.method, HoleKind::Method);
        visit_mut::visit_expr_method_call_mut(self, call);
    }

    fn visit_type_path_mut(&mut self, ty: &mut TypePath) {
        for segment in ty.path.segments.iter_mut() {
            self._hole(&mut segment.ident, HoleKind::Type);
        }
        visit_mut::visit_type_path_mut(self, ty);
    }

    fn visit_lit_mut(&mut self, lit: &mut Lit) {
        self.leaves.push(Leaf {
            kind: HoleKind::Literal,
            text: _lit_text(lit),
        });
        *lit = Lit::Str(LitStr::new("__hole", Span::call_site()));
    }
}

fn _lit_text(lit: &Lit) -> String {
    match lit {
        Lit::Str(v) => v.value(),
        Lit::Int(v) => v.base10_digits().to_string(),
        Lit::Float(v) => v.base10_digits().to_string(),
        Lit::Bool(v) => v.value.to_string(),
        Lit::Char(v) => v.value().to_string(),
        Lit::Byte(v) => v.value().to_string(),
        _ => String::new(),
    }
}
