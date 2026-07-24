use colored::Colorize;

use super::t::Group;

// not-obvious: this check is cross-file, so it does not use the shared
// `report::summary`
pub fn print(checked: usize, groups: Vec<Group>) -> bool {
    println!("{}", "duplicate-fns report:".bold().cyan());
    if groups.is_empty() {
        let success = "[success]".green().bold();
        println!(
            "{success} No alpha-equivalent functions found ({checked} functions checked)"
        );
        return false;
    }
    let duplicated: usize = groups.iter().map(|group| group.members.len()).sum();
    println!(
        "\n{}, {}\n",
        format!("{checked} functions checked").green(),
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
    println!("The following functions are alpha-equivalent duplicates:\n");
    for group in &groups {
        for member in &group.members {
            let location = format!("{}:{}", member.path, member.line);
            let key = format!("{}::{}", member.path, member.name);
            println!("  {} {}", location.bold(), key.red());
        }
        println!();
    }
}
