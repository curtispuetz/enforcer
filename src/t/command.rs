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
        let desc = self._desc();
        if desc.is_empty() {
            return String::new();
        }
        _unwrap_root(desc)
    }

    fn _desc(self) -> &'static str {
        match self {
            Command::All => "",
            Command::TreeStructure => include_str!("../rules/tree_structure/docs/desc.xml"),
            Command::FileSizes => include_str!("../rules/file_sizes/docs/desc.xml"),
            Command::Modules => include_str!("../rules/modules/docs/desc.xml"),
            Command::Comments => include_str!("../rules/comments/docs/desc.xml"),
            Command::Calls => include_str!("../rules/calls/docs/desc.xml"),
            Command::ModCount => include_str!("../rules/mod_count/docs/desc.xml"),
            Command::CognitiveComplexity => {
                include_str!("../rules/cognitive_complexity/docs/desc.xml")
            }
            Command::DuplicateFns => include_str!("../rules/duplicate_fns/docs/desc.xml"),
            Command::DuplicateLogic => include_str!("../rules/duplicate_logic/docs/desc.xml"),
            Command::PrivateFnNaming => include_str!("../rules/private_fn_naming/docs/desc.xml"),
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
