use {
    proc_macro2::Ident,
    std::collections::HashMap,
    syn::{
        ExprPath, Lifetime, PatIdent, Path, TypePath,
        visit_mut::{self, VisitMut},
    },
};

pub struct AlphaRenamer {
    pub values: HashMap<String, Ident>,
    pub lifetimes: HashMap<String, Ident>,
}

impl VisitMut for AlphaRenamer {
    fn visit_pat_ident_mut(&mut self, pat: &mut PatIdent) {
        if let Some(placeholder) = self.values.get(&pat.ident.to_string()) {
            pat.ident = placeholder.clone();
        }
        visit_mut::visit_pat_ident_mut(self, pat);
    }

    fn visit_expr_path_mut(&mut self, expr: &mut ExprPath) {
        _rename_single(&mut expr.path, &self.values);
        visit_mut::visit_expr_path_mut(self, expr);
    }

    fn visit_type_path_mut(&mut self, ty: &mut TypePath) {
        _rename_single(&mut ty.path, &self.values);
        visit_mut::visit_type_path_mut(self, ty);
    }

    fn visit_lifetime_mut(&mut self, lifetime: &mut Lifetime) {
        if let Some(placeholder) = self.lifetimes.get(&lifetime.ident.to_string()) {
            lifetime.ident = placeholder.clone();
        }
        visit_mut::visit_lifetime_mut(self, lifetime);
    }
}

fn _rename_single(path: &mut Path, values: &HashMap<String, Ident>) {
    if path.leading_colon.is_some() || path.segments.len() != 1 {
        return;
    }
    let segment = &mut path.segments[0];
    if !matches!(segment.arguments, syn::PathArguments::None) {
        return;
    }
    if let Some(placeholder) = values.get(&segment.ident.to_string()) {
        segment.ident = placeholder.clone();
    }
}
