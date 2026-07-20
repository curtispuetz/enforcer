use crate::import_rules::check::location::is_static_or_types;

pub fn is_import_allowed(use_path: &[String], file_dir: &[String]) -> bool {
    if use_path.iter().any(|s| is_static_or_types(s)) {
        return true;
    }
    if use_path.starts_with(file_dir) {
        return true;
    }
    allowed_through_c_dir(use_path, file_dir)
}

fn allowed_through_c_dir(use_path: &[String], file_dir: &[String]) -> bool {
    for (i, seg) in use_path.iter().enumerate() {
        if seg != "c" || !file_dir.starts_with(&use_path[..i]) {
            continue;
        }
        if file_dir.get(i).map(String::as_str) == Some("c") {
            return is_import_allowed(&use_path[i + 1..], &file_dir[i + 1..]);
        }
        return true;
    }
    false
}
