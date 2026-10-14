use {
    super::{count, home, room, suggestions},
    crate::{rules::t::Home, s::ROOT},
};

pub fn print<'a>(paths: impl IntoIterator<Item = &'a str>) {
    suggestions::print(&_fixes(paths));
}

fn _fixes<'a>(paths: impl IntoIterator<Item = &'a str>) -> Vec<String> {
    match home::of(paths) {
        None => Vec::new(),
        Some(Home::File(file)) => vec![format!("extract a private fn in {file}")],
        Some(Home::Crates) => vec![
            "the copies span tests/, whose files build as separate crates, so share it \
            from the library or from a tests/common/ module"
                .to_string(),
        ],
        Some(Home::Siblings { folder, files }) => _siblings(&folder, &files),
        Some(Home::Common {
            lowest,
            common,
            direct,
            kept,
        }) => _common(&lowest, &common, direct, kept),
    }
}

fn _siblings(folder: &str, files: &[String]) -> Vec<String> {
    let mut module = format!("put it in a new sibling module in {folder}/");
    if let Some(mod_rs) = ["mod.rs", "lib.rs"]
        .iter()
        .map(|name| ROOT.join(folder).join(name))
        .find(|file| file.is_file())
    {
        module =
            room::noted(&module, &mod_rs, &room::of(&mod_rs, count::mods(&mod_rs)));
    }
    module.push_str(" and import it with super::<module>::…");
    let names: Vec<&str> = files
        .iter()
        .filter(|file| !_root(file))
        .filter_map(|file| file.rsplit('/').next())
        .collect();
    let copy = match names.as_slice() {
        [] => return vec![module],
        [name] => format!("the copy in {name}"),
        names => format!("the copy in one of {}", names.join(", ")),
    };
    vec![
        module,
        format!(
            "or make {copy} pub and import it from the others with super::<file>::…"
        ),
    ]
}

fn _common(
    lowest: &[String],
    common: &[String],
    direct: bool,
    kept: Option<String>,
) -> Vec<String> {
    let dir = common.join("/");
    let target = if ROOT.join(&dir).join("mod.rs").is_file() {
        format!("{dir}/")
    } else if ROOT.join(format!("{dir}.rs")).is_file() {
        format!("{dir}.rs")
    } else {
        format!("{dir}/ (new)")
    };
    let module: Vec<&str> = common.iter().skip(1).map(String::as_str).collect();
    let mut line = format!(
        "extract it into {target} and import it with crate::{}::…",
        module.join("::")
    );
    if direct {
        let inner = common[lowest.len()..].join("::");
        let at = lowest.join("/");
        line.push_str(&format!(
            ", or super::{inner}::… from files directly in {at}/"
        ));
    }
    let mut lines = vec![line];
    lines.extend(
        kept.map(|file| {
            format!("or make the copy in {file} pub and import that instead")
        }),
    );
    lines
}

fn _root(file: &str) -> bool {
    file == "src/main.rs" || file == "src/lib.rs" || file.ends_with("/mod.rs")
}
