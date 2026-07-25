use std::{process, str::FromStr};

use enforcer::{help, run, t::Command};

fn main() {
    let args = _args();
    if _asks_for_help(&args) {
        _exit_help(&args);
    }
    if args.is_empty() {
        eprintln!("enforcer: no rule specified");
        eprintln!("usage: cargo enforcer <rule> [<rule>...]");
        _exit_unusable();
    }
    if _run_all(&args) {
        process::exit(1);
    }
}

fn _run_all(args: &[String]) -> bool {
    let mut any_failed = false;
    for command in _commands(args) {
        if run::rule(command) {
            any_failed = true;
        }
    }
    any_failed
}

fn _commands(args: &[String]) -> Vec<Command> {
    let mut commands = Vec::new();
    for name in args {
        match Command::from_str(name) {
            Ok(command) => commands.push(command),
            Err(_) => {
                eprintln!("enforcer: unknown rule '{name}'");
                _exit_unusable();
            }
        }
    }
    commands
}

fn _asks_for_help(args: &[String]) -> bool {
    args.iter()
        .any(|a| matches!(a.as_str(), "help" | "--help" | "-h"))
}

fn _exit_help(args: &[String]) -> ! {
    let rules: Vec<String> = args
        .iter()
        .filter(|a| !matches!(a.as_str(), "help" | "--help" | "-h"))
        .cloned()
        .collect();
    let ok = help::print(&rules);
    process::exit(if ok { 0 } else { 2 })
}

fn _exit_unusable() -> ! {
    eprintln!("available rules: {}", Command::available());
    process::exit(2)
}

// not-obvious: When invoked as `cargo enforcer <rule>`, cargo injects a leading
// `enforcer` argument, which we skip.
fn _args() -> Vec<String> {
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    if args.first().map(String::as_str) == Some("enforcer") {
        args.remove(0);
    }
    args
}
