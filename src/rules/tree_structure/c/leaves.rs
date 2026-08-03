use {
    crate::rules::tree_structure::t::SurfaceItem,
    std::path::Path,
    syn::{File, Ident, Item, Visibility},
};

pub fn of(ast: &File, file: &Path) -> Vec<SurfaceItem> {
    let mut ret = Vec::new();
    for item in &ast.items {
        if let Some((kind, name)) = _exported(item) {
            ret.push(SurfaceItem {
                kind,
                name,
                file: file.to_path_buf(),
            });
        }
    }
    ret
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
