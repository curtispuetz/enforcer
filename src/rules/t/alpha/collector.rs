use std::collections::HashMap;

use crate::rules::c::calls;
use proc_macro2::{Ident, Span};
use syn::{
    Expr, ExprCall, ExprPath, GenericParam, PatIdent,
    visit::{self, Visit},
};

#[derive(Default)]
pub struct AlphaCollector {
    pub values: HashMap<String, Ident>,
    pub lifetimes: HashMap<String, Ident>,
}

impl AlphaCollector {
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

impl<'ast> Visit<'ast> for AlphaCollector {
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

    fn visit_expr_path(&mut self, expr: &'ast ExprPath) {
        if let Some(name) = _bare_value_name(expr) {
            self._add_value(&name);
        }
        visit::visit_expr_path(self, expr);
    }

    // not-obvious: the callee of a bare call is a single-segment path too, and
    // renaming it would make `a()` and `b()` equivalent, so it is left unvisited
    fn visit_expr_call(&mut self, call: &'ast ExprCall) {
        if !matches!(*call.func, Expr::Path(_)) {
            self.visit_expr(&call.func);
        }
        for arg in &call.args {
            self.visit_expr(arg);
        }
    }
}

fn _bare_value_name(expr: &ExprPath) -> Option<String> {
    if expr.qself.is_some() || expr.path.leading_colon.is_some() {
        return None;
    }
    let [segment] = &expr.path.segments.iter().collect::<Vec<_>>()[..] else {
        return None;
    };
    if !matches!(segment.arguments, syn::PathArguments::None) {
        return None;
    }
    let name = segment.ident.to_string();
    (calls::is_function_like(&name) && name != "self").then_some(name)
}
