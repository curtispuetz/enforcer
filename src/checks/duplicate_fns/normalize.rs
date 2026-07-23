use std::collections::HashMap;

use proc_macro2::{Ident, Span};
use syn::{
    GenericParam, ItemFn, PatIdent, Visibility,
    visit::{self, Visit},
};

use crate::c::calls;

use super::rename;

pub fn canonical(item: &ItemFn) -> ItemFn {
    let mut collector = Collector::default();
    collector.visit_item_fn(item);

    let mut item = item.clone();
    rename::apply(&mut item, collector.values, collector.lifetimes);

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
