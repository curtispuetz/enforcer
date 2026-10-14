use {
    super::{imports, path},
    crate::rules::t::ModuleExports,
    std::{collections::HashSet, path::Path},
    syn::Item,
};

pub fn table(file: &Path, ast: &syn::File) -> ModuleExports {
    let mut ret = ModuleExports::default();
    for item in &ast.items {
        if let Item::Mod(m) = item {
            ret.children.insert(m.ident.to_string());
        } else {
            ret.items.extend(_defined(item));
        }
    }
    let (named, globs) = imports::reexports(ast);
    for (binding, use_path) in named {
        if let Some(target) = _absolute(&use_path, file, &ret.children) {
            ret.named.insert(binding, target);
        }
    }
    ret.globs = globs
        .iter()
        .filter_map(|glob| _absolute(glob, file, &ret.children))
        .collect();
    ret
}

fn _defined(item: &Item) -> Option<String> {
    let ident = match item {
        Item::Fn(i) => &i.sig.ident,
        Item::Static(i) => &i.ident,
        Item::Const(i) => &i.ident,
        Item::Struct(i) => &i.ident,
        Item::Enum(i) => &i.ident,
        Item::Union(i) => &i.ident,
        Item::Type(i) => &i.ident,
        Item::Trait(i) => &i.ident,
        Item::Macro(i) => i.ident.as_ref()?,
        _ => return None,
    };
    Some(ident.to_string())
}

// not-obvious: a `pub use` path that starts with a declared child module is relative
// to this module; any other bare first segment is an external crate.
fn _absolute(
    use_path: &[String],
    file: &Path,
    children: &HashSet<String>,
) -> Option<Vec<String>> {
    if !children.contains(use_path.first()?) {
        return path::absolute(use_path, file);
    }
    let mut ret = path::module(file)?;
    ret.extend_from_slice(use_path);
    Some(ret)
}
