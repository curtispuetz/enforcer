use std::path::Path;

use syn::{File, Ident, Item, Visibility};

use crate::checks::c::ast;
use crate::checks::tree_structure::c::path;

pub fn misplaced(path: &Path, file: &File) -> Vec<String> {
    let mut ret = Vec::new();
    for item in &file.items {
        if let Some((common, desc)) = _home(item)
            && !common.iter().any(|c| path::in_common(path, c))
        {
            ret.push(format!("{desc} -> {}", common.join(" or ")));
        }
    }
    ret
}

fn _home(item: &Item) -> Option<(&'static [&'static str], String)> {
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
    common: &'static [&'static str],
    kind: &str,
    vis: &Visibility,
    ident: &Ident,
) -> Option<(&'static [&'static str], String)> {
    ast::is_public(vis).then(|| (common, format!("{kind} {ident}")))
}
