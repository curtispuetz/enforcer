use crate::rules::{c::ast, private_fn_naming::t::Misnamed};

pub fn misnamed(file: &syn::File) -> Misnamed {
    let mut found = Misnamed {
        free: Vec::new(),
        methods: Vec::new(),
    };
    for item in &file.items {
        _check_item(item, &mut found);
    }
    found
}

fn _check_item(item: &syn::Item, found: &mut Misnamed) {
    match item {
        syn::Item::Fn(f) => _push(&f.vis, &f.sig.ident, &mut found.free),
        syn::Item::Impl(imp) if imp.trait_.is_none() => {
            for impl_item in &imp.items {
                if let syn::ImplItem::Fn(f) = impl_item {
                    _push(&f.vis, &f.sig.ident, &mut found.methods);
                }
            }
        }
        _ => {}
    }
}

fn _push(vis: &syn::Visibility, ident: &syn::Ident, out: &mut Vec<String>) {
    let name = ident.to_string();
    if !ast::is_public(vis) && name != "main" && !name.starts_with('_') {
        out.push(name);
    }
}
