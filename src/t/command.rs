use strum::{Display, EnumIter, EnumString, IntoEnumIterator};

use crate::import_rules;

#[derive(Clone, Copy, Display, EnumIter, EnumString)]
#[strum(serialize_all = "kebab-case")]
pub enum Command {
    ImportRules,
}

impl Command {
    pub fn run(self) {
        match self {
            Command::ImportRules => import_rules::run(),
        }
    }

    pub fn available() -> String {
        let mut names = Vec::new();
        for command in Command::iter() {
            names.push(command.to_string());
        }
        names.join(", ")
    }
}
