use std::collections::HashMap;

use crate::rules::c::calls;
use proc_macro2::{Ident, Span};
use syn::{
    GenericParam, PatIdent,
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
}
