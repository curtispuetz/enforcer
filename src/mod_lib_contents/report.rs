use crate::{c::report, mod_lib_contents::t::violation::Violation};

pub fn report(passed: usize, violations: Vec<Violation>) -> bool {
    report::summary(
        "mod-lib-contents",
        passed,
        violations,
        || {
            println!(
                "[success] All mod.rs and lib.rs files contain only mod and use statements ({passed} files checked)"
            )
        },
        _print_failures,
    )
}

fn _print_failures(violations: Vec<Violation>) {
    println!(
        "The following mod.rs or lib.rs file(s) contain statements other than mod and use:\n"
    );
    for violation in &violations {
        println!("  {} ({})", violation.path, violation.items.join(", "));
    }
}
