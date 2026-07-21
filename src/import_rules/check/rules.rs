use crate::import_rules::check::location::is_commons;

pub fn is_import_allowed(use_path: &[String], file_dir: &[String]) -> bool {
    if use_path.starts_with(file_dir) {
        return true;
    }
    _allowed_through_commons_dir(use_path, file_dir)
}

fn _allowed_through_commons_dir(use_path: &[String], file_dir: &[String]) -> bool {
    for (i, seg) in use_path.iter().enumerate() {
        if !is_commons(seg) || !file_dir.starts_with(&use_path[..i]) {
            continue;
        }
        if file_dir.get(i) == Some(seg) {
            return is_import_allowed(&use_path[i + 1..], &file_dir[i + 1..]);
        }
        return true;
    }
    false
}
