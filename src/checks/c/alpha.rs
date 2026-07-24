use std::collections::HashMap;

use proc_macro2::{Ident, Span};
use syn::{
    Block, ExprPath, GenericParam, ItemFn, Lifetime, PatIdent, Path, Stmt, TypePath,
    Visibility,
    visit::{self, Visit},
    visit_mut::{self, VisitMut},
};

use super::calls;

pub fn canonical_item_fn(item: &ItemFn) -> ItemFn {
    let mut out = item.clone();
    _canonicalize(
        &mut out,
        |c, node| c.visit_item_fn(node),
        |r, node| r.visit_item_fn_mut(node),
    );
    out.attrs.clear();
    out.vis = Visibility::Inherited;
    out.sig.ident = Ident::new("__fn", Span::call_site());
    out
}

pub fn canonical_block(stmts: &[Stmt]) -> Block {
    let mut out = Block {
        brace_token: Default::default(),
        stmts: stmts.to_vec(),
    };
    _canonicalize(
        &mut out,
        |c, node| c.visit_block(node),
        |r, node| r.visit_block_mut(node),
    );
    out
}

fn _canonicalize<N>(
    node: &mut N,
    collect: impl Fn(&mut _Collector, &N),
    rename: impl Fn(&mut _Renamer, &mut N),
) {
    let mut collector = _Collector::default();
    collect(&mut collector, node);
    let mut renamer = _Renamer {
        values: collector.values,
        lifetimes: collector.lifetimes,
    };
    rename(&mut renamer, node);
}

#[derive(Default)]
struct _Collector {
    values: HashMap<String, Ident>,
    lifetimes: HashMap<String, Ident>,
}

impl _Collector {
    fn _add_value(&mut self, name: &str) {
        if !self.values.contains_key(name) {
            let placeholder =
                Ident::new(&format!("__v{}", self.values.len()), Span::call_site());
            self.values.insert(name.to_string(), placeholder);
        }
    }

    fn _add_lifetime(&mut self, name: &str) {
        if !self.lifetimes.contains_key(name) {
            let placeholder =
                Ident::new(&format!("__l{}", self.lifetimes.len()), Span::call_site());
            self.lifetimes.insert(name.to_string(), placeholder);
        }
    }
}

impl<'ast> Visit<'ast> for _Collector {
    fn visit_generic_param(&mut self, param: &'ast GenericParam) {
        match param {
            GenericParam::Type(type_param) => {
                self._add_value(&type_param.ident.to_string())
            }
            GenericParam::Const(const_param) => {
                self._add_value(&const_param.ident.to_string())
            }
            GenericParam::Lifetime(lifetime) => {
                self._add_lifetime(&lifetime.lifetime.ident.to_string())
            }
        }
        visit::visit_generic_param(self, param);
    }

    fn visit_pat_ident(&mut self, pat: &'ast PatIdent) {
        let name = pat.ident.to_string();
        if calls::is_function_like(&name) {
            self._add_value(&name);
        }
        visit::visit_pat_ident(self, pat);
    }
}

struct _Renamer {
    values: HashMap<String, Ident>,
    lifetimes: HashMap<String, Ident>,
}

impl VisitMut for _Renamer {
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
