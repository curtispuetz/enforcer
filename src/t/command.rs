use strum::{Display, EnumIter, EnumString, IntoEnumIterator};

#[derive(Clone, Copy, Display, EnumIter, EnumString)]
#[strum(serialize_all = "kebab-case")]
pub enum Command {
    All,
    ImportRules,
    FileSizes,
    ModLocation,
    ModOverFile,
    ModLibContents,
    CommentRules,
    TCommons,
    CallRules,
    CommonsReexport,
    CommonsItems,
    ModCount,
    UsePrivacy,
    CognitiveComplexity,
    DuplicateFns,
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
