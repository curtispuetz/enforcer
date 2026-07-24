use syn::{Ident, Item, Visibility};

use crate::c::ast;

pub fn of(item: &Item) -> Option<(&'static [&'static str], String)> {
    match item {
        Item::Struct(i) => _public(&["t"], "struct", &i.vis, &i.ident),
        Item::Enum(i) => _public(&["t"], "enum", &i.vis, &i.ident),
        Item::Type(i) => _public(&["t"], "type", &i.vis, &i.ident),
        Item::Static(i) => _public(&["s"], "static", &i.vis, &i.ident),
        Item::Const(i) => _public(&["cnst"], "const", &i.vis, &i.ident),
        Item::Trait(i) => _public(&["t", "ext_traits"], "trait", &i.vis, &i.ident),
        _ => None,
    }
}

fn _public(
    commons: &'static [&'static str],
    kind: &str,
    vis: &Visibility,
    ident: &Ident,
) -> Option<(&'static [&'static str], String)> {
    ast::is_public(vis).then(|| (commons, format!("{kind} {ident}")))
}
