use {
    super::t::{Group, Hole},
    crate::rules::c::report,
    colored::Colorize,
};

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
    println!("The following logic fragments are duplicates once identifiers are");
    println!("normalized and any holes below are made helper parameters.");
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
        _print_holes(&group.holes);
        println!();
    }
}

fn _print_holes(holes: &[Hole]) {
    if holes.is_empty() {
        return;
    }
    println!(
        "    {}",
        format!("{} hole(s), so the shared helper takes:", holes.len()).blue()
    );
    for hole in holes {
        println!("      {} <- {}", hole.kind, hole.values.join(" | "));
    }
}
