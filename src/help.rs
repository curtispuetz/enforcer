use colored::Colorize;

use super::t::Command;

pub fn print(names: &[String]) -> bool {
    if names.is_empty() {
        _usage();
        return true;
    }
    let mut ok = true;
    for name in names {
        match name.parse::<Command>() {
            Ok(Command::All) => _usage(),
            Ok(command) => _desc(command),
            Err(_) => {
                eprintln!("{} unknown rule '{name}'", "enforcer:".red().bold());
                ok = false;
            }
        }
    }
    ok
}

fn _usage() {
    println!("{}", "usage: cargo enforcer <rule> [<rule>...]".bold());
    println!("{}", "       cargo enforcer help <rule>".bold());
    println!("available rules: {}", Command::available());
}

fn _desc(command: Command) {
    println!("{}", command.to_string().cyan().bold());
    println!("{}", command.desc().trim_end());
}
