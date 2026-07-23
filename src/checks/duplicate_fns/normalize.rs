use std::collections::HashMap;

use proc_macro2::{Ident, Span};
use syn::{
    ExprPath, GenericParam, ItemFn, Lifetime, PatIdent, Path, TypePath, Visibility,
    visit::{self, Visit},
    visit_mut::{self, VisitMut},
};

use crate::c::calls;

pub fn canonical(item: &ItemFn) -> ItemFn {
    let mut collector = Collector::default();
    collector.visit_item_fn(item);

    let mut item = item.clone();
    let mut renamer = Renamer {
        values: collector.values,
        lifetimes: collector.lifetimes,
    };
    renamer.visit_item_fn_mut(&mut item);

    item.attrs.clear();
    item.vis = Visibility::Inherited;
    item.sig.ident = Ident::new("__fn", Span::call_site());
    item
}

#[derive(Default)]
struct Collector {
    values: HashMap<String, Ident>,
    lifetimes: HashMap<String, Ident>,
}

impl Collector {
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

impl<'ast> Visit<'ast> for Collector {
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

struct Renamer {
    values: HashMap<String, Ident>,
    lifetimes: HashMap<String, Ident>,
}

impl VisitMut for Renamer {
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
