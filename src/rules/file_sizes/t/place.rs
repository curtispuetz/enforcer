use {crate::rules::t::ChildLinks, std::path::PathBuf};

pub enum Place {
    Declared {
        mod_rs: PathBuf,
        name: String,
        links: Vec<ChildLinks>,
    },
    Parent {
        mod_rs: PathBuf,
        mods: usize,
    },
    Loose,
}
