use strum::IntoEnumIterator;

use super::t::Command;

use super::checks;

pub fn check(command: Command) -> bool {
    let ret = match command {
        Command::All => _all(),
        Command::ImportRules => checks::import_rules::run(),
        Command::FileSizes => checks::file_sizes::run(),
        Command::ModLocation => checks::mod_location::run(),
        Command::ModOverFile => checks::mod_over_file::run(),
        Command::ModLibContents => checks::mod_lib_contents::run(),
        Command::CommentRules => checks::comment_rules::run(),
        Command::TypeLocation => checks::type_location::run(),
        Command::ImplLocation => checks::impl_location::run(),
        Command::ContainedT => checks::contained_t::run(),
        Command::StaticLocation => checks::static_location::run(),
        Command::CallRules => checks::call_rules::run(),
        Command::CModContents => checks::c_mod_contents::run(),
        Command::CommonsNesting => checks::commons_nesting::run(),
        Command::CommonsExports => checks::commons_exports::run(),
        Command::ModCount => checks::mod_count::run(),
        Command::UsePrivacy => checks::use_privacy::run(),
        Command::CognitiveComplexity => checks::cognitive_complexity::run(),
        Command::TraitRules => checks::trait_rules::run(),
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
