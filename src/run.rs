use {
    super::{
        rules,
        s::FILE_CONFIG,
        t::{Command, Mode},
    },
    colored::Colorize,
    strum::IntoEnumIterator,
};

pub fn rules(commands: &[Command], mode: Mode) -> bool {
    match mode {
        Mode::Check => _check(commands),
        Mode::Fix => _for_each(commands, _fix),
        Mode::Report => _for_each(commands, _report),
    }
}

pub fn rule(command: Command) -> bool {
    let ret = match command {
        Command::All => _all(),
        Command::TreeStructure => rules::tree_structure::run(),
        Command::FileSizes => rules::file_sizes::run(),
        Command::ModuleFiles => rules::module_files::run(),
        Command::Comments => rules::comments::run(),
        Command::Calls => rules::calls::run(),
        Command::ModCount => rules::mod_count::run(),
        Command::CognitiveComplexity => rules::cognitive_complexity::run(),
        Command::DuplicateFns => rules::duplicate_fns::run(),
        Command::DuplicateLogic => rules::duplicate_logic::run(),
        Command::PrivateFnNaming => rules::private_fn_naming::run(),
    };
    if ret || FILE_CONFIG.debug {
        println!();
    }
    ret
}

fn _check(commands: &[Command]) -> bool {
    let any_failed = _each(commands);
    if !any_failed && !FILE_CONFIG.debug {
        println!("{} All rules passed", "[Success]".green().bold());
    }
    any_failed
}

fn _for_each(commands: &[Command], action: fn(Command)) -> bool {
    for command in commands {
        match command {
            Command::All => {
                _for_each(&_others(), action);
            }
            _ => action(*command),
        }
    }
    false
}

fn _fix(command: Command) {
    if matches!(command, Command::PrivateFnNaming) {
        rules::private_fn_naming::fix();
    }
}

fn _report(command: Command) {
    if matches!(command, Command::CognitiveComplexity) {
        rules::cognitive_complexity::report();
    }
}

fn _all() -> bool {
    _each(&_others())
}

fn _others() -> Vec<Command> {
    Command::iter()
        .filter(|command| !matches!(command, Command::All))
        .collect()
}

fn _each(commands: &[Command]) -> bool {
    let mut any_failed = false;
    for command in commands {
        if rule(*command) {
            any_failed = true;
        }
    }
    any_failed
}
