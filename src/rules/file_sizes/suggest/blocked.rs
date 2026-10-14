use {
    crate::rules::{
        c::{ast, child, files, path, suggestions},
        s::COMMON,
        t::{ChildLinks, Importer},
    },
    std::{collections::BTreeSet, path::Path},
    syn::Item,
};

pub fn lines(
    head: &str,
    mod_rs: &Path,
    owner: &ChildLinks,
    owned: &BTreeSet<String>,
    blocked: &[&ChildLinks],
) -> Vec<String> {
    let inside = |name: &String| name == &owner.name || owned.contains(name);
    let reasons: Vec<String> = blocked
        .iter()
        .map(|link| _reason(mod_rs, link, &inside))
        .collect();
    let label = |link: &&ChildLinks| child::label(mod_rs, &link.name);
    let callers: Vec<String> = blocked.iter().map(label).collect();
    let fns: Vec<String> = blocked
        .iter()
        .filter(|link| link.imports.is_empty() && _fns_only(mod_rs, &link.name))
        .map(label)
        .collect();
    let folder = mod_rs.parent().map(path::rel).unwrap_or_default();
    let mut way = format!(
        "way out: keep the code that calls {} in a sibling file directly in {folder}/",
        callers.join(", ")
    );
    if !fns.is_empty() && !_in_common(mod_rs) {
        way.push_str(&format!(", or move {} into {folder}/c/", fns.join(", ")));
    }
    let up = format!("{}/main.rs would import up to", owner.name);
    vec![format!("{head}: blocked, {up} {}", reasons.join(", ")), way]
}

fn _reason(mod_rs: &Path, link: &ChildLinks, inside: &impl Fn(&String) -> bool) -> String {
    let label = child::label(mod_rs, &link.name);
    let users: Vec<String> = link
        .importers
        .iter()
        .filter(|importer| !matches!(importer, Importer::Sibling(name) if inside(name)))
        .map(|importer| child::importer(mod_rs, importer))
        .collect();
    if !users.is_empty() {
        return format!("{label} (also used by {})", suggestions::shortlist(&users));
    }
    let needs: Vec<String> = link
        .imports
        .iter()
        .filter(|name| !inside(name))
        .map(|name| child::label(mod_rs, name))
        .collect();
    if needs.is_empty() {
        return label;
    }
    format!("{label} (needs {}, left behind)", needs.join(", "))
}

fn _in_common(mod_rs: &Path) -> bool {
    mod_rs
        .parent()
        .and_then(Path::file_name)
        .is_some_and(|name| COMMON.contains(&name.to_string_lossy().as_ref()))
}

fn _fns_only(mod_rs: &Path, name: &str) -> bool {
    child::file(mod_rs, name)
        .is_some_and(|file| files::ast_parse(&file).items.iter().all(_fn_surface))
}

fn _fn_surface(item: &Item) -> bool {
    let vis = match item {
        Item::Fn(_) | Item::Impl(_) | Item::Macro(_) => return true,
        Item::Use(i) => &i.vis,
        Item::Const(i) => &i.vis,
        Item::Static(i) => &i.vis,
        Item::Struct(i) => &i.vis,
        Item::Enum(i) => &i.vis,
        Item::Type(i) => &i.vis,
        Item::Trait(i) => &i.vis,
        _ => return false,
    };
    !ast::is_public(vis)
}
