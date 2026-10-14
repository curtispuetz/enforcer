use {
    super::t::{Folder, Member},
    crate::rules::{
        c::{path, suggestions},
        t::FileImports,
    },
};

pub fn lines(
    folder: &Folder,
    members: &[&Member],
    importers: &[&FileImports],
) -> Vec<String> {
    let it = if members.len() == 1 { "it" } else { "them" };
    let beside = if importers.len() == 1 {
        "its importer"
    } else {
        "its importers"
    };
    let dest = folder.path.parent().map(path::rel).unwrap_or_default();
    let mut texts = vec![format!(
        "move {it} out of `c` into {dest}/, beside {beside}"
    )];
    if let [only] = importers
        && !path::is_mod_or_lib(&only.path)
    {
        texts.push(format!(
            "or nest {it} under {}, its only importer",
            path::rel(&only.path)
        ));
    }
    suggestions::lines(&texts)
}
