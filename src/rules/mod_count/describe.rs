use {super::t::Fix, crate::rules::c::suggestions};

pub fn fix(fix: &Fix) -> String {
    let frees = fix.frees();
    let text = match fix {
        Fix::Merge { from, into, lines } => format!(
            "merge {from}.rs into {into}.rs ({}+{} lines; {from}'s pub fns become private _fns)",
            lines.0, lines.1
        ),
        Fix::Nest { owner, owned, room } => {
            format!(
                "nest {} under {owner}/ ({})",
                owned.join(", "),
                _nest_how(owner, *room)
            )
        }
        Fix::Group { members, repoint } => {
            format!(
                "group {} into a new folder{}",
                members.join(", "),
                _repoint(repoint)
            )
        }
    };
    format!("{text}: frees {frees}")
}

fn _nest_how(owner: &str, room: Option<usize>) -> String {
    match room {
        Some(room) => format!("{owner}/mod.rs has {room} free slots"),
        None => format!(
            "{owner}.rs -> {owner}/main.rs, with {owner}/mod.rs re-exporting what siblings use"
        ),
    }
}

fn _repoint(repoint: &[String]) -> String {
    if repoint.is_empty() {
        return String::new();
    }
    format!(" (repoint imports in {})", suggestions::shortlist(repoint))
}
