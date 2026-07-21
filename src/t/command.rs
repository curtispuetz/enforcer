use strum::{Display, EnumIter, EnumString, IntoEnumIterator};

use crate::{comment_rules, file_sizes, import_rules, mod_lib_contents, mod_location};

#[derive(Clone, Copy, Display, EnumIter, EnumString)]
#[strum(serialize_all = "kebab-case")]
pub enum Command {
    ImportRules,
    FileSizes,
    ModLocation,
    ModLibContents,
    CommentRules,
}

impl Command {
    pub fn run(self) -> bool {
        let ret = match self {
            Command::ImportRules => import_rules::run(),
            Command::FileSizes => file_sizes::run(),
            Command::ModLocation => mod_location::run(),
            Command::ModLibContents => mod_lib_contents::run(),
            Command::CommentRules => comment_rules::run(),
        };
        println!();
        ret
    }

    pub fn available() -> String {
        let mut names = Vec::new();
        for command in Command::iter() {
            names.push(command.to_string());
        }
        names.join(", ")
    }
}
