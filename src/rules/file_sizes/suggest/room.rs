use {
    crate::rules::{
        c::path,
        file_sizes::t::{Config, Room},
    },
    std::path::Path,
};

pub fn of(mod_rs: &Path, count: usize, config: &Config) -> Room {
    if config.mod_ignore.contains(&_folder(mod_rs)) {
        Room::Uncapped
    } else if count < config.mod_max {
        Room::Free(config.mod_max - count)
    } else {
        Room::Full {
            count,
            max: config.mod_max,
        }
    }
}

pub fn text(mod_rs: &Path, room: &Room) -> String {
    let file = path::rel(mod_rs);
    match room {
        Room::Uncapped => format!("{} is exempt from mod-count", _folder(mod_rs)),
        Room::Free(free) => format!("{file} has {} free", slots(*free)),
        Room::Full { count, max } if count > max => format!(
            "but {file} is full, {count} of max {max} modules; see mod-count's suggestions for {}",
            _folder(mod_rs)
        ),
        Room::Full { max, .. } => format!(
            "but {file} is full, {max} of max {max} modules; make room by nesting, grouping or merging modules"
        ),
    }
}

pub fn slots(count: usize) -> String {
    let plural = if count == 1 { "" } else { "s" };
    format!("{count} slot{plural}")
}

fn _folder(mod_rs: &Path) -> String {
    mod_rs.parent().map(path::rel).unwrap_or_default()
}
