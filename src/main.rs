use std::{process, str::FromStr};

use enforcer::{run, t::Command};

fn main() {
    let names = _subcommands();
    if names.is_empty() {
        eprintln!("enforcer: no check specified");
        eprintln!("usage: cargo enforcer <check> [<check>...]");
        _exit_unusable();
    }
    let mut commands = Vec::new();
    for name in &names {
        match Command::from_str(name) {
            Ok(command) => commands.push(command),
            Err(_) => {
                eprintln!("enforcer: unknown check '{name}'");
                _exit_unusable();
            }
        }
    }
    let mut all_passed = true;
    for command in commands {
        if run::check(command) {
            all_passed = false;
        }
    }
    if !all_passed {
        process::exit(1);
    }
}

fn _exit_unusable() -> ! {
    eprintln!("available checks: {}", Command::available());
    process::exit(2)
}

// not-obvious: When invoked as `cargo enforcer <check>`, cargo injects a leading
// `enforcer` argument, which we skip.
fn _subcommands() -> Vec<String> {
    let mut positional: Vec<String> = std::env::args()
        .skip(1)
        .filter(|a| !a.starts_with('-'))
        .collect();
    if positional.first().map(String::as_str) == Some("enforcer") {
        positional.remove(0);
    }
    positional
}
