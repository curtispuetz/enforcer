use colored::Colorize;

use super::t::Group;

// This check is cross-file (a violation spans several files), so it does not use
// the shared per-file `report::summary`; it mirrors its colour scheme instead.
pub fn print(passed: usize, groups: Vec<Group>) -> bool {
    println!("{}", "duplicate-fns report:".bold().cyan());
    if groups.is_empty() {
        let success = "[success]".green().bold();
        println!(
            "{success} No alpha-equivalent free functions found ({passed} functions checked)"
        );
        return false;
    }
    let duplicated: usize = groups.iter().map(|group| group.members.len()).sum();
    println!(
        "\n{}, {}\n",
        format!("{passed} functions passed").green(),
        format!(
            "{duplicated} duplicate functions in {} group(s)",
            groups.len()
        )
        .red()
        .bold()
    );
    _print_groups(groups);
    true
}

fn _print_groups(groups: Vec<Group>) {
    println!("The following free functions are alpha-equivalent duplicates:\n");
    for group in &groups {
        for member in &group.members {
            let location = format!("{}:{}", member.path, member.line);
            let key = format!("{}::{}", member.path, member.name);
            println!("  {} {}", location.bold(), key.red());
        }
        println!();
    }
}
