use crate::{c::report, mod_location::t::violation::Violation};

pub fn report(passed: usize, violations: Vec<Violation>) -> bool {
    report::summary(
        "mod-location",
        passed,
        violations,
        || {
            println!(
                "All mod statements are in mod.rs or lib.rs files ({passed} files checked)"
            )
        },
        _print_failures,
    )
}

fn _print_failures(violations: Vec<Violation>) {
    println!(
        "The following file(s) have mod statements but are not mod.rs or lib.rs:\n"
    );
    for violation in &violations {
        println!(
            "  {} (mod {})",
            violation.path,
            violation.mods.join(", mod ")
        );
    }
}
