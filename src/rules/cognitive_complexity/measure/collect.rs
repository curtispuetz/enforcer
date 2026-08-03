use {
    super::score,
    crate::rules::{c::fn_name, cognitive_complexity::t::Function},
    syn::{Block, ImplItem, Item, TraitItem},
};

pub fn functions(file: &syn::File) -> Vec<Function> {
    let mut found = Vec::new();
    _items(&file.items, None, &mut found);
    found
}

fn _items(items: &[Item], type_name: Option<&str>, found: &mut Vec<Function>) {
    for item in items {
        _item(item, type_name, found);
    }
}

fn _item(item: &Item, type_name: Option<&str>, found: &mut Vec<Function>) {
    match item {
        Item::Fn(f) => _measure(&f.sig, &f.block, type_name, found),
        Item::Mod(m) => _module(m, type_name, found),
        Item::Impl(i) => _impl_fns(i, found),
        Item::Trait(t) => _trait_fns(t, found),
        _ => {}
    }
}

fn _module(m: &syn::ItemMod, type_name: Option<&str>, found: &mut Vec<Function>) {
    if let Some((_, items)) = &m.content {
        _items(items, type_name, found);
    }
}

fn _impl_fns(i: &syn::ItemImpl, found: &mut Vec<Function>) {
    let self_name = fn_name::self_type(&i.self_ty);
    for impl_item in &i.items {
        if let ImplItem::Fn(m) = impl_item {
            _measure(&m.sig, &m.block, self_name.as_deref(), found);
        }
    }
}

fn _trait_fns(t: &syn::ItemTrait, found: &mut Vec<Function>) {
    for trait_item in &t.items {
        if let TraitItem::Fn(m) = trait_item
            && let Some(block) = &m.default
        {
            _measure(&m.sig, block, Some(&t.ident.to_string()), found);
        }
    }
}

fn _measure(
    sig: &syn::Signature,
    block: &Block,
    type_name: Option<&str>,
    found: &mut Vec<Function>,
) {
    found.push(Function {
        name: fn_name::of(&sig.ident, type_name),
        line: sig.ident.span().start().line,
        score: score::of(block),
    });
}
