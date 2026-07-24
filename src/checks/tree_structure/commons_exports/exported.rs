use std::{collections::HashSet, path::Path};

use crate::c::{ast, files};

pub fn glob_modules(mod_rs: &Path) -> HashSet<String> {
    let mut modules = HashSet::new();
    for item in files::ast_parse(mod_rs).items {
        if let syn::Item::Use(u) = &item
            && ast::is_public(&u.vis)
            && let Some(module) = ast::glob_module(&u.tree)
        {
            modules.insert(module);
        }
    }
    modules
}

pub fn has_public_items(mod_rs: &Path, module: &str) -> bool {
    let Some(file) = _module_file(mod_rs, module) else {
        return true;
    };
    files::ast_parse(&file).items.iter().any(_is_public_item)
}

fn _module_file(mod_rs: &Path, module: &str) -> Option<std::path::PathBuf> {
    let dir = mod_rs.parent()?;
    let sibling = dir.join(format!("{module}.rs"));
    if sibling.is_file() {
        return Some(sibling);
    }
    let nested = dir.join(module).join("mod.rs");
    nested.is_file().then_some(nested)
}

fn _is_public_item(item: &syn::Item) -> bool {
    match item {
        syn::Item::Const(i) => ast::is_public(&i.vis),
        syn::Item::Enum(i) => ast::is_public(&i.vis),
        syn::Item::Fn(i) => ast::is_public(&i.vis),
        syn::Item::Mod(i) => ast::is_public(&i.vis),
        syn::Item::Static(i) => ast::is_public(&i.vis),
        syn::Item::Struct(i) => ast::is_public(&i.vis),
        syn::Item::Trait(i) => ast::is_public(&i.vis),
        syn::Item::TraitAlias(i) => ast::is_public(&i.vis),
        syn::Item::Type(i) => ast::is_public(&i.vis),
        syn::Item::Union(i) => ast::is_public(&i.vis),
        syn::Item::Use(i) => ast::is_public(&i.vis),
        _ => false,
    }
}
