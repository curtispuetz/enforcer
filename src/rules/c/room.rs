use {
    super::path,
    crate::{rules::t::Room, s::FILE_CONFIG},
    std::path::Path,
};

pub fn of(mod_rs: &Path, count: usize) -> Room {
    let max = FILE_CONFIG.mod_count.max;
    if path::ignore_set(&FILE_CONFIG.mod_count.ignore).contains(&_folder(mod_rs)) {
        Room::Uncapped
    } else if count < max {
        Room::Free(max - count)
    } else {
        Room::Full
    }
}

fn _note(mod_rs: &Path, room: &Room) -> Option<String> {
    let file = path::rel(mod_rs);
    match room {
        Room::Uncapped => Some(format!("{} is exempt from mod-count", _folder(mod_rs))),
        Room::Free(free) => Some(format!("{file} has {} free", slots(*free))),
        Room::Full => None,
    }
}

pub fn noted(text: &str, mod_rs: &Path, room: &Room) -> String {
    match _note(mod_rs, room) {
        Some(note) => format!("{text} ({note})"),
        None => text.to_string(),
    }
}

pub fn slots(count: usize) -> String {
    let plural = if count == 1 { "" } else { "s" };
    format!("{count} slot{plural}")
}

fn _folder(mod_rs: &Path) -> String {
    mod_rs.parent().map(path::rel).unwrap_or_default()
}
