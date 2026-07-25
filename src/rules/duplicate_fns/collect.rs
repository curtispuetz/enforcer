use syn::{ImplItem, ImplItemFn, Item, ItemFn};

use crate::{
    rules::c::{files, fn_name, path},
    s::EXISTING_SRC_DIRS,
};

use super::t::CollectedFn;

pub fn all_codebase_fns() -> Vec<CollectedFn> {
    let mut found = Vec::new();
    for dir_name in EXISTING_SRC_DIRS.iter() {
        for file_path in files::rs(dir_name) {
            let file = files::ast_parse(&file_path);
            let relative = path::rel(&file_path);
            _items(&file.items, &relative, &mut found);
        }
    }
    found
}

fn _items(items: &[Item], relative: &str, found: &mut Vec<CollectedFn>) {
    for item in items {
        match item {
            Item::Fn(function) => _push(found, relative, None, function.clone()),
            Item::Mod(module) => {
                if let Some((_, items)) = &module.content {
                    _items(items, relative, found);
                }
            }
            // not-obvious: trait impl methods are skipped because the trait
            // dictates their shape, making the boilerplate unavoidable
            Item::Impl(block) if block.trait_.is_none() => {
                _methods(block, relative, found);
            }
            _ => {}
        }
    }
}

fn _methods(block: &syn::ItemImpl, relative: &str, found: &mut Vec<CollectedFn>) {
    let owner = fn_name::self_type(&block.self_ty);
    for impl_item in &block.items {
        if let ImplItem::Fn(method) = impl_item {
            _push(found, relative, owner.as_deref(), _as_item_fn(method));
        }
    }
}

fn _push(
    found: &mut Vec<CollectedFn>,
    relative: &str,
    owner: Option<&str>,
    item: ItemFn,
) {
    found.push(CollectedFn {
        path: relative.to_string(),
        name: fn_name::of(&item.sig.ident, owner),
        line: item.sig.ident.span().start().line,
        item,
    });
}

fn _as_item_fn(method: &ImplItemFn) -> ItemFn {
    ItemFn {
        attrs: method.attrs.clone(),
        vis: method.vis.clone(),
        modifiers: method.modifiers.clone(),
        sig: method.sig.clone(),
        block: Box::new(method.block.clone()),
    }
}
