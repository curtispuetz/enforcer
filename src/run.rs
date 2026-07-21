use super::{
    comment_rules, file_sizes, import_rules, mod_lib_contents, mod_location,
    t::command::Command,
};

pub fn check(command: Command) -> bool {
    let ret = match command {
        Command::ImportRules => import_rules::run(),
        Command::FileSizes => file_sizes::run(),
        Command::ModLocation => mod_location::run(),
        Command::ModLibContents => mod_lib_contents::run(),
        Command::CommentRules => comment_rules::run(),
    };
    println!();
    ret
}
