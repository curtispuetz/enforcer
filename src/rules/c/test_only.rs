use {
    super::{files, path},
    std::path::{Path, PathBuf},
    syn::{Attribute, Meta, Token, punctuated::Punctuated},
};

pub fn attrs(attrs: &[Attribute]) -> bool {
    attrs.iter().any(|attr| {
        attr.path().is_ident("cfg")
            && attr
                .parse_args::<Meta>()
                .is_ok_and(|meta| _clearly_test(&meta))
    })
}

pub fn file(path: &Path) -> bool {
    if attrs(&files::ast_parse(path).attrs) {
        return true;
    }
    let Some((name, decl_dir)) = _decl_site(path) else {
        return false;
    };
    _declarers(&decl_dir, path)
        .into_iter()
        .find_map(|declarer| _mod_attrs(&declarer, &name).map(|a| (declarer, a)))
        .is_some_and(|(declarer, a)| attrs(&a) || file(&declarer))
}

fn _clearly_test(meta: &Meta) -> bool {
    match meta {
        Meta::Path(p) => p.is_ident("test"),
        Meta::List(list) => {
            let Ok(args) =
                list.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
            else {
                return false;
            };
            if list.path.is_ident("all") {
                args.iter().any(_clearly_test)
            } else {
                list.path.is_ident("any")
                    && !args.is_empty()
                    && args.iter().all(_clearly_test)
            }
        }
        Meta::NameValue(_) => false,
    }
}

fn _decl_site(path: &Path) -> Option<(String, PathBuf)> {
    let dir = path.parent()?;
    if path::is_mod_or_lib(path) {
        let name = dir.file_name()?.to_str()?.to_string();
        Some((name, dir.parent()?.to_path_buf()))
    } else {
        let name = path.file_stem()?.to_str()?.to_string();
        Some((name, dir.to_path_buf()))
    }
}

fn _declarers(decl_dir: &Path, path: &Path) -> Vec<PathBuf> {
    let mut candidates: Vec<PathBuf> = ["mod.rs", "lib.rs", "main.rs"]
        .iter()
        .map(|f| decl_dir.join(f))
        .collect();
    if let Some(stem) = decl_dir.file_name() {
        candidates
            .push(decl_dir.with_file_name(format!("{}.rs", stem.to_string_lossy())));
    }
    candidates
        .into_iter()
        .filter(|c| c != path && c.is_file())
        .collect()
}

fn _mod_attrs(declarer: &Path, name: &str) -> Option<Vec<Attribute>> {
    files::ast_parse(declarer)
        .items
        .into_iter()
        .find_map(|item| match item {
            syn::Item::Mod(m) if m.ident == name => Some(m.attrs),
            _ => None,
        })
}
