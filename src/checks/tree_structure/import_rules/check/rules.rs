use crate::checks::s::COMMON;

enum Form {
    Crate,
    Super,
}

pub fn import_violation(
    use_path: &[String],
    file_dir: &[String],
    own_module: &[String],
) -> Option<&'static str> {
    let (form, resolved) = _resolve(use_path, own_module)?;
    let sideways_or_deeper = resolved.starts_with(file_dir);
    match form {
        Form::Super if sideways_or_deeper => None,
        Form::Super => Some("`super` must not go up, only sideways or deeper"),
        Form::Crate if sideways_or_deeper => {
            Some("sideways or deeper import must use `super::`, not `crate::`")
        }
        Form::Crate if _allowed_through_common_dir(&resolved, file_dir) => None,
        Form::Crate => Some("import reaches up or across the module tree"),
    }
}

fn _resolve(use_path: &[String], own_module: &[String]) -> Option<(Form, Vec<String>)> {
    match use_path.first()?.as_str() {
        "crate" => Some((Form::Crate, use_path.to_vec())),
        "super" => Some((Form::Super, _resolved_super(use_path, own_module))),
        _ => None,
    }
}

fn _resolved_super(use_path: &[String], own_module: &[String]) -> Vec<String> {
    let supers = use_path.iter().take_while(|seg| *seg == "super").count();
    let base = own_module.len().saturating_sub(supers);
    _joined(&own_module[..base], &use_path[supers..])
}

fn _joined(base: &[String], rest: &[String]) -> Vec<String> {
    let mut ret = base.to_vec();
    ret.extend_from_slice(rest);
    ret
}

fn _within_or_common(use_path: &[String], file_dir: &[String]) -> bool {
    use_path.starts_with(file_dir) || _allowed_through_common_dir(use_path, file_dir)
}

fn _allowed_through_common_dir(use_path: &[String], file_dir: &[String]) -> bool {
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

fn _is_common(name: &str) -> bool {
    COMMON.contains(&name)
}
