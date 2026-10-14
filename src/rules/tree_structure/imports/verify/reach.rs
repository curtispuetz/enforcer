use crate::rules::s::COMMON;

pub fn through_common(use_path: &[String], file_dir: &[String]) -> bool {
    for (i, seg) in use_path.iter().enumerate() {
        if !_is_common(seg) || !file_dir.starts_with(&use_path[..i]) {
            continue;
        }
        if file_dir.get(i) == Some(seg) {
            return _within_or_common(&use_path[i + 1..], &file_dir[i + 1..]);
        }
        return true;
    }
    false
}

fn _within_or_common(use_path: &[String], file_dir: &[String]) -> bool {
    use_path.starts_with(file_dir) || through_common(use_path, file_dir)
}

fn _is_common(name: &str) -> bool {
    COMMON.contains(&name)
}
