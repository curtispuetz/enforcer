use {
    super::{rules, s::FILE_CONFIG, t::Command},
    colored::Colorize,
    strum::IntoEnumIterator,
};

pub fn rules(commands: &[Command], fix: bool) -> bool {
    if fix {
        _fix_each(commands);
        return false;
    }
    let any_failed = _each(commands);
    if !any_failed && !FILE_CONFIG.debug {
        println!("{} All rules passed", "[Success]".green().bold());
    }
    any_failed
}

pub fn rule(command: Command) -> bool {
    let ret = match command {
        Command::All => _all(),
        Command::TreeStructure => rules::tree_structure::run(),
        Command::FileSizes => rules::file_sizes::run(),
        Command::ModLocation => rules::mod_location::run(),
        Command::ModOverFile => rules::mod_over_file::run(),
        Command::ModLibContents => rules::mod_lib_contents::run(),
        Command::Comments => rules::comments::run(),
        Command::Calls => rules::calls::run(),
        Command::ModCount => rules::mod_count::run(),
        Command::UsePrivacy => rules::use_privacy::run(),
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

fn _fix_each(commands: &[Command]) {
    for command in commands {
        match command {
            Command::All => _fix_each(&_others()),
            Command::PrivateFnNaming => rules::private_fn_naming::fix(),
            _ => {}
        }
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
