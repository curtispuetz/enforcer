use strum::{Display, EnumIter, EnumString, IntoEnumIterator};

#[derive(Clone, Copy, Display, EnumIter, EnumString)]
#[strum(serialize_all = "kebab-case")]
pub enum Command {
    All,
    TreeStructure,
    FileSizes,
    Modules,
    Comments,
    Calls,
    ModCount,
    CognitiveComplexity,
    DuplicateFns,
    DuplicateLogic,
    PrivateFnNaming,
}

impl Command {
    pub fn available() -> String {
        let mut names = Vec::new();
        for command in Command::iter() {
            names.push(command.to_string());
        }
        names.join(", ")
    }

    pub fn help(self) -> String {
        let Some((desc, rest)) = self._docs().split_first() else {
            return String::new();
        };
        let mut parts = vec![_unwrap_root(desc)];
        parts.extend(rest.iter().map(|doc| doc.trim().to_string()));
        parts.join("\n")
    }

    fn _docs(self) -> &'static [&'static str] {
        match self {
            Command::All => &[],
            Command::TreeStructure => &[
                include_str!("../rules/tree_structure/docs/desc.xml"),
                include_str!("../rules/tree_structure/docs/config.xml"),
            ],
            Command::FileSizes => &[
                include_str!("../rules/file_sizes/docs/desc.xml"),
                include_str!("../rules/file_sizes/docs/config.xml"),
            ],
            Command::Modules => &[include_str!("../rules/modules/docs/desc.xml")],
            Command::Comments => &[
                include_str!("../rules/comments/docs/desc.xml"),
                include_str!("../rules/comments/docs/config.xml"),
            ],
            Command::Calls => &[
                include_str!("../rules/calls/docs/desc.xml"),
                include_str!("../rules/calls/docs/config.xml"),
            ],
            Command::ModCount => &[
                include_str!("../rules/mod_count/docs/desc.xml"),
                include_str!("../rules/mod_count/docs/config.xml"),
            ],
            Command::CognitiveComplexity => &[
                include_str!("../rules/cognitive_complexity/docs/desc.xml"),
                include_str!("../rules/cognitive_complexity/docs/scoring.xml"),
                include_str!("../rules/cognitive_complexity/docs/config.xml"),
            ],
            Command::DuplicateFns => &[
                include_str!("../rules/duplicate_fns/docs/desc.xml"),
                include_str!("../rules/duplicate_fns/docs/config.xml"),
            ],
            Command::DuplicateLogic => &[
                include_str!("../rules/duplicate_logic/docs/desc.xml"),
                include_str!("../rules/duplicate_logic/docs/holes.xml"),
                include_str!("../rules/duplicate_logic/docs/config.xml"),
            ],
            Command::PrivateFnNaming => &[include_str!("../rules/private_fn_naming/docs/desc.xml")],
        }
    }
}

fn _unwrap_root(desc: &str) -> String {
    let lines: Vec<&str> = desc.trim().lines().collect();
    let Some(inner) = lines.get(1..lines.len().saturating_sub(1)) else {
        return desc.trim().to_string();
    };
    let dedented = inner.iter().map(|line| line.strip_prefix("  ").unwrap_or(line));
    dedented.collect::<Vec<_>>().join("\n")
}
