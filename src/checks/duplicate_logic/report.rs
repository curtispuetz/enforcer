use colored::Colorize;

use super::t::Group;

// not-obvious: this check is cross-file, so it does not use the shared
// `report::summary`
pub fn print(scanned: usize, groups: Vec<Group>) -> bool {
    println!("{}", "duplicate-logic report:".bold().cyan());
    if groups.is_empty() {
        let success = "[success]".green().bold();
        println!(
            "{success} No duplicated logic fragments found ({scanned} fragments scanned)"
        );
        return false;
    }
    let occurrences: usize = groups.iter().map(|group| group.occurrences.len()).sum();
    println!(
        "\n{}, {}\n",
        format!("{scanned} fragments scanned").green(),
        format!(
            "{occurrences} duplicated fragments in {} group(s)",
            groups.len()
        )
        .red()
        .bold()
    );
    _print_groups(groups);
    true
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
