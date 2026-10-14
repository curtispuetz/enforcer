use super::t::Fix;

const MAX_REPOINT_SHOWN: usize = 3;

pub fn fix(fix: &Fix) -> String {
    let frees = fix.frees();
    let text = match fix {
        Fix::Merge { from, into, lines } => format!(
            "merge {from}.rs into {into}.rs ({}+{} lines; {from}'s pub fns become private _fns)",
            lines.0, lines.1
        ),
        Fix::Nest { owner, owned, room } => {
            format!(
                "nest {} under {owner} ({})",
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
    format!("suggested fix: {text}: frees {frees}")
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
    let shown: Vec<&str> = repoint
        .iter()
        .take(MAX_REPOINT_SHOWN)
        .map(String::as_str)
        .collect();
    let more = repoint.len().saturating_sub(MAX_REPOINT_SHOWN);
    let tail = if more > 0 {
        format!(" +{more} more")
    } else {
        String::new()
    };
    format!(" (repoint imports in {}{tail})", shown.join(", "))
}
