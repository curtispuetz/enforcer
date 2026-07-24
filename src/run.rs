use strum::IntoEnumIterator;

use super::t::Command;

use super::checks;

pub fn check(command: Command) -> bool {
    let ret = match command {
        Command::All => _all(),
        Command::TreeStructure => checks::tree_structure::run(),
        Command::FileSizes => checks::file_sizes::run(),
        Command::ModLocation => checks::mod_location::run(),
        Command::ModOverFile => checks::mod_over_file::run(),
        Command::ModLibContents => checks::mod_lib_contents::run(),
        Command::CommentRules => checks::comment_rules::run(),
        Command::CallRules => checks::call_rules::run(),
        Command::ModCount => checks::mod_count::run(),
        Command::UsePrivacy => checks::use_privacy::run(),
        Command::CognitiveComplexity => checks::cognitive_complexity::run(),
        Command::DuplicateFns => checks::duplicate_fns::run(),
        Command::DuplicateLogic => checks::duplicate_logic::run(),
        Command::PrivateFnNaming => checks::private_fn_naming::run(),
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
        if check(command) {
            any_failed = true;
        }
    }
    any_failed
}
