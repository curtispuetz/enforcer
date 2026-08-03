use {crate::rules::c::files, std::path::Path};

pub fn mods(path: &Path) -> usize {
    let mut count = 0;
    for item in files::ast_parse(path).items {
        if matches!(item, syn::Item::Mod(_)) {
            count += 1;
        }
    }
    count
}
