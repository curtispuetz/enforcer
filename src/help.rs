use {super::t::Command, colored::Colorize};

pub fn print(names: &[String]) -> bool {
    if names.is_empty() {
        _usage();
        return true;
    }
    let mut ok = true;
    for name in names {
        match name.parse::<Command>() {
            Ok(Command::All) => _usage(),
            Ok(command) => _help(command),
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
    println!("{}", "--fix: apply the available automated fixes".bold());
    println!("{}", "--report: print the available reports".bold());
}

fn _help(command: Command) {
    println!("{}", command.to_string().cyan().bold());
    println!("{}", command.help().trim_end());
}
