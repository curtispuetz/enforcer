use std::path::Path;

use crate::c::{ast, files};

use super::{defined, exported, t::Kind};

pub fn of(mod_rs: &Path, kind: Kind) -> Vec<String> {
    let mut issues = _public_modules(mod_rs);
    issues.extend(_missing_exports(mod_rs, kind));
    issues
}

fn _public_modules(mod_rs: &Path) -> Vec<String> {
    let mut issues = Vec::new();
    for item in files::parse(mod_rs).items {
        if let syn::Item::Mod(m) = &item
            && ast::is_public(&m.vis)
        {
            issues.push(format!("module `{}` is not private", m.ident));
        }
    }
    issues
}

fn _missing_exports(mod_rs: &Path, kind: Kind) -> Vec<String> {
    let (glob, exported) = exported::names(mod_rs);
    if glob {
        return Vec::new();
    }
    let mut issues = Vec::new();
    for name in defined::public_names(mod_rs, kind) {
        if !exported.contains(&name) {
            issues.push(format!(
                "public {} `{}` is not re-exported in mod.rs",
                kind.noun(),
                name
            ));
        }
    }
    issues
}
