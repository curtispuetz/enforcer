use {
    super::{exports, files, path},
    crate::rules::{
        s::COMMON,
        t::{ChildLinks, FileImports, Importer},
    },
    std::{collections::BTreeSet, path::Path},
};

pub fn of(folder: &Path, index: &[FileImports]) -> Vec<ChildLinks> {
    let Some(module) = path::module(folder)
        .filter(|_| folder.is_file() && path::is_mod_or_lib(folder))
    else {
        return Vec::new();
    };
    let ast = files::ast_parse(folder);
    let exports = exports::table(folder, &ast);
    let reexported: Vec<Vec<String>> =
        exports.named.into_values().chain(exports.globs).collect();
    let names: Vec<String> = ast
        .items
        .iter()
        .filter_map(|item| match item {
            syn::Item::Mod(m) => Some(m.ident.to_string()),
            _ => None,
        })
        .collect();
    names
        .iter()
        .map(|name| _links(&module, name, &names, &reexported, index))
        .collect()
}

fn _links(
    folder: &[String],
    name: &str,
    names: &[String],
    reexported: &[Vec<String>],
    index: &[FileImports],
) -> ChildLinks {
    let mut segments = folder.to_vec();
    segments.push(name.to_string());
    let imports = index
        .iter()
        .filter(|file| file.module.starts_with(&segments))
        .flat_map(|file| &file.resolved)
        .filter(|target| !target.starts_with(&segments))
        .filter_map(|target| _child(folder, target))
        .filter(|child| names.contains(child) && !COMMON.contains(&child.as_str()))
        .cloned()
        .collect();
    let mut importers: BTreeSet<Importer> = index
        .iter()
        .filter(|file| !file.module.starts_with(&segments))
        .filter(|file| file.resolved.iter().any(|t| t.starts_with(&segments)))
        .map(|file| _importer(folder, file, names))
        .collect();
    if reexported.iter().any(|t| t.starts_with(&segments)) {
        importers.insert(Importer::Parent);
    }
    ChildLinks {
        name: name.to_string(),
        is_common: COMMON.contains(&name),
        imports,
        importers,
    }
}

fn _importer(folder: &[String], file: &FileImports, names: &[String]) -> Importer {
    if file.module == folder {
        return Importer::Parent;
    }
    match _child(folder, &file.module) {
        Some(child) if names.contains(child) => Importer::Sibling(child.clone()),
        _ => Importer::Elsewhere(file.path.clone()),
    }
}

fn _child<'a>(folder: &[String], module: &'a [String]) -> Option<&'a String> {
    module.strip_prefix(folder)?.first()
}
