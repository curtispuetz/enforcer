use proc_macro2::{Ident, TokenStream};
use syn::visit::{self, Visit};

use super::Misnamed;

pub struct Finder<'a> {
    pub names: &'a Misnamed,
    pub spots: Vec<(String, usize, usize)>,
    pub bound: Vec<String>,
    pub macros: Vec<TokenStream>,
}

impl<'a> Visit<'a> for Finder<'a> {
    fn visit_item_fn(&mut self, node: &'a syn::ItemFn) {
        if self.names.free.contains(&node.sig.ident.to_string()) {
            _push(&mut self.spots, &node.sig.ident);
        }
        visit::visit_item_fn(self, node);
    }

    fn visit_impl_item_fn(&mut self, node: &'a syn::ImplItemFn) {
        if self.names.methods.contains(&node.sig.ident.to_string()) {
            _push(&mut self.spots, &node.sig.ident);
        }
        visit::visit_impl_item_fn(self, node);
    }

    fn visit_path(&mut self, node: &'a syn::Path) {
        if let Some(segment) = node.segments.last()
            && _named(self.names, &segment.ident.to_string())
        {
            _push(&mut self.spots, &segment.ident);
        }
        visit::visit_path(self, node);
    }

    fn visit_expr_method_call(&mut self, node: &'a syn::ExprMethodCall) {
        if self.names.methods.contains(&node.method.to_string()) {
            _push(&mut self.spots, &node.method);
        }
        visit::visit_expr_method_call(self, node);
    }

    fn visit_pat_ident(&mut self, node: &'a syn::PatIdent) {
        self.bound.push(node.ident.to_string());
        visit::visit_pat_ident(self, node);
    }

    fn visit_macro(&mut self, node: &'a syn::Macro) {
        self.macros.push(node.tokens.clone());
        visit::visit_macro(self, node);
    }
}

fn _named(names: &Misnamed, name: &String) -> bool {
    names.free.contains(name) || names.methods.contains(name)
}

fn _push(spots: &mut Vec<(String, usize, usize)>, ident: &Ident) {
    let start = ident.span().start();
    spots.push((ident.to_string(), start.line, start.column));
}
