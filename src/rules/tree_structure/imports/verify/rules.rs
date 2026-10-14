use {
    super::{reach, rewrite},
    crate::rules::tree_structure::imports::t::Fix,
};

enum Form {
    Crate,
    Super,
}

pub fn import_violation(
    use_path: &[String],
    file_dir: &[String],
    own_module: &[String],
) -> Option<(&'static str, Option<Fix>)> {
    let (form, resolved) = _resolve(use_path, own_module)?;
    let sideways_or_deeper = resolved.starts_with(file_dir);
    match form {
        Form::Super if sideways_or_deeper => None,
        Form::Super => Some((
            "`super` must not go up, only sideways or deeper",
            _super_up_fix(&resolved, file_dir),
        )),
        Form::Crate if sideways_or_deeper => Some((
            "sideways or deeper import must use `super::`, not `crate::`",
            rewrite::relative(&resolved, file_dir, own_module),
        )),
        Form::Crate if reach::through_common(&resolved, file_dir) => None,
        Form::Crate => Some((
            "import reaches up or across the module tree",
            Some(Fix::MoveToCommon),
        )),
    }
}

fn _super_up_fix(resolved: &[String], file_dir: &[String]) -> Option<Fix> {
    if resolved.first().map(String::as_str) != Some("crate") {
        return None;
    }
    if reach::through_common(resolved, file_dir) {
        return Some(rewrite::absolute(resolved));
    }
    Some(Fix::MoveToCommon)
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
