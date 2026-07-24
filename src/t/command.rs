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
}
