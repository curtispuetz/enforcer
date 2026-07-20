mod c;
mod check;
mod macros;
mod report;
mod s;
mod t;

use std::process;

use t::config::Config;

use crate::s::main::EXISTING_SRC_DIRS;

fn main() {
    match subcommand().as_deref() {
        Some("import-rules") => import_rules(),
        Some(other) => {
            eprintln!("enforcer: unknown check '{other}'");
            eprintln!("available checks: import-rules");
            process::exit(2);
        }
        None => {
            eprintln!("enforcer: no check specified");
            eprintln!("usage: cargo enforcer <check>");
            eprintln!("available checks: import-rules");
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

fn import_rules() {
    let config = Config::new();
    let (passed, failed) = check_all(&config);
    report::report(passed, failed);
}

fn check_all(config: &Config) -> (usize, usize) {
    let mut passed = 0;
    let mut failed = 0;
    for dir_name in EXISTING_SRC_DIRS.iter() {
        let (p, f) = check::dir(dir_name, config);
        passed += p;
        failed += f;
    }
    (passed, failed)
}
