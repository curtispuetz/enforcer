use std::{process, str::FromStr};

use enforcer::t::command::Command;

fn main() {
    match subcommand().as_deref() {
        Some(name) => match Command::from_str(name) {
            Ok(command) => command.run(),
            Err(_) => {
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

// non-obvious: When invoked as `cargo enforcer <check>`, cargo injects a leading
// `enforcer` argument, which we skip.
fn subcommand() -> Option<String> {
    let mut positional = std::env::args().skip(1).filter(|a| !a.starts_with('-'));
    match positional.next() {
        Some(arg) if arg == "enforcer" => positional.next(),
        other => other,
    }
}
