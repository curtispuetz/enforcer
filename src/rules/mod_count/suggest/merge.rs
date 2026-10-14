use {
    super::child,
    crate::rules::{
        c::files,
        mod_count::t::Fix,
        s::COMMON,
        t::{ChildLinks, Importer},
    },
    std::path::Path,
};

pub fn all(mod_rs: &Path, links: &[ChildLinks], max_lines: usize) -> Vec<Fix> {
    child::movable(links)
        .into_iter()
        .filter_map(|link| _merge(mod_rs, link, max_lines))
        .collect()
}

fn _merge(mod_rs: &Path, link: &ChildLinks, max_lines: usize) -> Option<Fix> {
    if link.importers.len() != 1 {
        return None;
    }
    let Some(Importer::Sibling(into)) = link.importers.first() else {
        return None;
    };
    if COMMON.contains(&into.as_str()) || link.imports.iter().any(|name| name != into) {
        return None;
    }
    let lines = (
        files::lines(&child::file(mod_rs, &link.name)?),
        files::lines(&child::file(mod_rs, into)?),
    );
    (lines.0 + lines.1 <= max_lines).then(|| Fix::Merge {
        from: link.name.clone(),
        into: into.clone(),
        lines,
    })
}
