use strum::IntoEnumIterator;

use super::t::Command;

use super::rules;

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
    println!();
    ret
}

fn _all() -> bool {
    let mut any_failed = false;
    for command in Command::iter() {
        if matches!(command, Command::All) {
            continue;
        }
        if rule(command) {
            any_failed = true;
        }
    }
    any_failed
}
