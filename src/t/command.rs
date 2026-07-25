use strum::{Display, EnumIter, EnumString, IntoEnumIterator};

#[derive(Clone, Copy, Display, EnumIter, EnumString)]
#[strum(serialize_all = "kebab-case")]
pub enum Command {
    All,
    TreeStructure,
    FileSizes,
    ModLocation,
    ModOverFile,
    ModLibContents,
    CommentRules,
    CallRules,
    ModCount,
    UsePrivacy,
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

    pub fn desc(self) -> &'static str {
        match self {
            Command::All => "",
            Command::TreeStructure => include_str!("../rules/tree_structure/desc.xml"),
            Command::FileSizes => include_str!("../rules/file_sizes/desc.xml"),
            Command::ModLocation => include_str!("../rules/mod_location/desc.xml"),
            Command::ModOverFile => include_str!("../rules/mod_over_file/desc.xml"),
            Command::ModLibContents => {
                include_str!("../rules/mod_lib_contents/desc.xml")
            }
            Command::CommentRules => include_str!("../rules/comment_rules/desc.xml"),
            Command::CallRules => include_str!("../rules/call_rules/desc.xml"),
            Command::ModCount => include_str!("../rules/mod_count/desc.xml"),
            Command::UsePrivacy => include_str!("../rules/use_privacy/desc.xml"),
            Command::CognitiveComplexity => {
                include_str!("../rules/cognitive_complexity/desc.xml")
            }
            Command::DuplicateFns => include_str!("../rules/duplicate_fns/desc.xml"),
            Command::DuplicateLogic => {
                include_str!("../rules/duplicate_logic/desc.xml")
            }
            Command::PrivateFnNaming => {
                include_str!("../rules/private_fn_naming/desc.xml")
            }
        }
    }
}
