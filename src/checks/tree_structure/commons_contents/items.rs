use std::path::Path;

use syn::{Ident, Item, Visibility};

use crate::c::files;

pub fn disallowed(path: &Path, commons: &str) -> Vec<String> {
    let mut ret = Vec::new();
    for item in files::ast_parse(path).items {
        if let Some((kind, name)) = _exported(&item)
            && !_allowed(commons, kind)
        {
            ret.push(format!("{kind} {name}"));
        }
    }
    ret
}

fn _allowed(commons: &str, kind: &str) -> bool {
    match commons {
        "t" => matches!(kind, "struct" | "enum" | "type" | "trait"),
        "s" => kind == "static",
        "cnst" => kind == "const",
        "ext_traits" => kind == "trait",
        _ => kind == "fn",
    }
}

fn _exported(item: &Item) -> Option<(&'static str, String)> {
    match item {
        Item::Fn(i) => _entry("fn", &i.vis, &i.sig.ident),
        Item::Static(i) => _entry("static", &i.vis, &i.ident),
        Item::Const(i) => _entry("const", &i.vis, &i.ident),
        Item::Struct(i) => _entry("struct", &i.vis, &i.ident),
        Item::Enum(i) => _entry("enum", &i.vis, &i.ident),
        Item::Union(i) => _entry("union", &i.vis, &i.ident),
        Item::Type(i) => _entry("type", &i.vis, &i.ident),
        Item::Trait(i) => _entry("trait", &i.vis, &i.ident),
        _ => None,
    }
}

fn _entry(
    kind: &'static str,
    vis: &Visibility,
    ident: &Ident,
) -> Option<(&'static str, String)> {
    _escapes(vis).then(|| (kind, ident.to_string()))
}

fn _escapes(vis: &Visibility) -> bool {
    match vis {
        Visibility::Public(_) => true,
        Visibility::Restricted(r) => r.path.is_ident("crate"),
        Visibility::Inherited => false,
    }
}
