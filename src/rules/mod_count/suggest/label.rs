use {
    super::child,
    crate::rules::mod_count::t::Fix,
    std::path::Path,
};

pub fn all(mod_rs: &Path, fixes: Vec<Fix>) -> Vec<Fix> {
    fixes.into_iter().map(|fix| _fix(mod_rs, fix)).collect()
}

fn _fix(mod_rs: &Path, fix: Fix) -> Fix {
    match fix {
        Fix::Nest { owner, owned, room } => Fix::Nest {
            owner,
            owned: _names(mod_rs, owned),
            room,
        },
        Fix::Group { members, repoint } => Fix::Group {
            members: _names(mod_rs, members),
            repoint,
        },
        merge => merge,
    }
}

fn _names(mod_rs: &Path, names: Vec<String>) -> Vec<String> {
    names.iter().map(|name| child::label(mod_rs, name)).collect()
}
