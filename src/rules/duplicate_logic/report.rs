use colored::Colorize;

use crate::rules::c::report;

use super::t::Group;

// not-obvious: this rule is cross-file, so its counts are fragments and groups
// rather than the files `report::summary` assumes
pub fn print(scanned: usize, groups: Vec<Group>) -> bool {
    let occurrences: usize = groups.iter().map(|group| group.occurrences.len()).sum();
    report::counted(
        "duplicate-logic",
        &format!("No duplicated logic fragments found ({scanned} fragments scanned)"),
        &format!("{scanned} fragments scanned"),
        &format!(
            "{occurrences} duplicated fragments in {} group(s)",
            groups.len()
        ),
        groups,
        _print_groups,
    )
}

fn _print_groups(groups: Vec<Group>) {
    println!("The following logic fragments are alpha-equivalent duplicates.");
    println!(
        "Add an id to `[duplicate-logic] ignore` in enforcer.toml to silence one:\n"
    );
    for group in &groups {
        let header = format!("id: {}", group.id);
        println!("  {}", header.yellow().bold());
        for occ in &group.occurrences {
            let location = format!("{}:{}-{}", occ.path, occ.start, occ.end);
            println!("    {}", location.red());
        }
        println!();
    }
}
