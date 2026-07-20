use std::process;

use enforcer::t::command::Command;

fn main() {
    match subcommand().as_deref() {
        Some(name) => match Command::parse(name) {
            Some(command) => command.run(),
            None => {
                eprintln!("enforcer: unknown check '{name}'");
                eprintln!("available checks: {}", Command::available());
                process::exit(2);
            }
        },
        None => {
            eprintln!("enforcer: no check specified");
            eprintln!("usage: cargo enforcer <check>");
            eprintln!("available checks: {}", Command::available());
            process::exit(2);
        }
    }
}

/// Returns the requested check name. When invoked as `cargo enforcer <check>`,
/// cargo injects a leading `enforcer` argument, which we skip.
fn subcommand() -> Option<String> {
    let mut positional = std::env::args().skip(1).filter(|a| !a.starts_with('-'));
    match positional.next() {
        Some(arg) if arg == "enforcer" => positional.next(),
        other => other,
    }
}
