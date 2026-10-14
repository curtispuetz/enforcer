use {crate::rules::t::Home, std::collections::BTreeSet};

pub fn of<'a>(paths: impl IntoIterator<Item = &'a str>) -> Option<Home> {
    let files: BTreeSet<&str> = paths.into_iter().collect();
    let files: Vec<String> = files.into_iter().map(String::from).collect();
    if let [file] = files.as_slice() {
        return Some(Home::File(file.clone()));
    }
    if files.iter().any(|file| file.starts_with("tests/")) {
        return Some(Home::Crates);
    }
    let dirs: Vec<Vec<String>> = files.iter().map(|file| _dir(file)).collect();
    let lowest = _lowest(&dirs)?;
    if dirs.iter().all(|dir| dir.len() == lowest.len()) {
        let folder = lowest.join("/");
        return Some(Home::Siblings { folder, files });
    }
    let mut common = lowest.clone();
    common.push("c".to_string());
    // not-obvious: a copy below the `c` folder can't import up into it, so the
    // home moves into that `c`'s own `c`, which the folder re-roots around.
    while dirs
        .iter()
        .any(|dir| dir.len() > common.len() && dir.starts_with(&common))
    {
        common.push("c".to_string());
    }
    let kept = files.iter().zip(&dirs).find(|(_, dir)| **dir == common);
    Some(Home::Common {
        direct: dirs.iter().any(|dir| dir.len() == lowest.len()),
        kept: kept.map(|(file, _)| file.clone()),
        lowest,
        common,
    })
}

fn _dir(file: &str) -> Vec<String> {
    let mut parts: Vec<String> = file.split('/').map(String::from).collect();
    parts.pop();
    parts
}

fn _lowest(dirs: &[Vec<String>]) -> Option<Vec<String>> {
    let first = dirs.first()?;
    let shared = (0..first.len())
        .take_while(|&i| dirs.iter().all(|dir| dir.get(i) == first.get(i)))
        .count();
    Some(first[..shared].to_vec())
}
