use {super::t::Group, crate::rules::c::report, colored::Colorize};

// not-obvious: this rule is cross-file, so its counts are functions and groups
// rather than the files `report::summary` assumes
pub fn print(checked: usize, groups: Vec<Group>) -> bool {
    let duplicated: usize = groups.iter().map(|group| group.members.len()).sum();
    report::counted(
        "duplicate-fns",
        &format!("No duplicate functions found ({checked} functions checked)"),
        &format!("{checked} functions checked"),
        &format!(
            "{duplicated} duplicate functions in {} group(s)",
            groups.len()
        ),
        groups,
        _print_groups,
    )
}

fn _print_groups(groups: Vec<Group>) {
    println!("The following functions are identifier-normalized duplicates:\n");
    for group in &groups {
        for member in &group.members {
            let location = format!("{}:{}", member.path, member.line);
            let key = format!("{}::{}", member.path, member.name);
            println!("  {} {}", location.bold(), key.red());
        }
        println!();
    }
}
