use crate::import_rules;

#[derive(Clone, Copy)]
pub enum Command {
    ImportRules,
}

impl Command {
    pub const ALL: &'static [Command] = &[Command::ImportRules];

    pub fn name(self) -> &'static str {
        match self {
            Command::ImportRules => "import-rules",
        }
    }

    pub fn run(self) {
        match self {
            Command::ImportRules => import_rules::run(),
        }
    }

    pub fn parse(name: &str) -> Option<Command> {
        for command in Command::ALL {
            if command.name() == name {
                return Some(*command);
            }
        }
        None
    }

    pub fn available() -> String {
        let mut names = Vec::new();
        for command in Command::ALL {
            names.push(command.name());
        }
        names.join(", ")
    }
}
