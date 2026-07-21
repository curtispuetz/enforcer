use strum::{Display, EnumIter, EnumString, IntoEnumIterator};

use crate::{file_sizes, import_rules};

#[derive(Clone, Copy, Display, EnumIter, EnumString)]
#[strum(serialize_all = "kebab-case")]
pub enum Command {
    ImportRules,
    FileSizes,
}

impl Command {
    pub fn run(self) -> bool {
        let ret = match self {
            Command::ImportRules => import_rules::run(),
            Command::FileSizes => file_sizes::run(),
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
