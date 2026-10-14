use {
    crate::s::{FILE_CONFIG, ROOT},
    std::{
        collections::HashSet,
        path::{Component, Path, PathBuf},
    },
};

pub fn ignore_set(entries: &[String]) -> HashSet<String> {
    entries.iter().map(|e| e.replace('\\', "/")).collect()
}

pub fn ignored(path: &Path) -> bool {
    let rel = rel(path);
    FILE_CONFIG.ignore.iter().any(|i| {
        let i = i.trim_start_matches("./").trim_end_matches('/');
        !i.is_empty() && (rel == i || rel.starts_with(&format!("{i}/")))
    })
}

pub fn rel(path: &Path) -> String {
    path.strip_prefix(ROOT.as_path())
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

pub fn is_mod_or_lib(path: &Path) -> bool {
    matches!(
        path.file_name().and_then(|n| n.to_str()),
        Some("mod.rs" | "lib.rs")
    )
}

pub fn file_dir(path: &Path) -> Option<Vec<String>> {
    let rel = path.strip_prefix(ROOT.as_path()).ok()?;
    let mut segments = vec!["crate".to_string()];
    // not-obvious: skip(1) drops the top-level src dir (src/ or tests/).
    for component in rel.parent()?.components().skip(1) {
        if let Component::Normal(s) = component {
            segments.push(s.to_str()?.to_string());
        }
    }
    Some(segments)
}

pub fn module(path: &Path) -> Option<Vec<String>> {
    let mut segments = file_dir(path)?;
    if !is_mod_or_lib(path) {
        segments.push(path.file_stem()?.to_str()?.to_string());
    }
    Some(segments)
}

pub fn absolute(segments: &[String], file: &Path) -> Option<Vec<String>> {
    let mut acc: Vec<String> = Vec::new();
    for (i, seg) in segments.iter().enumerate() {
        match seg.as_str() {
            "crate" if i == 0 => acc = vec!["crate".to_string()],
            "self" if i == 0 => acc = module(file)?,
            "super" => {
                if i == 0 {
                    acc = module(file)?;
                }
                acc.pop()?;
            }
            _ => acc.push(seg.clone()),
        }
    }
    (acc.first().map(String::as_str) == Some("crate")).then_some(acc)
}

pub fn child(file: &Path, name: &str) -> Option<PathBuf> {
    if !is_mod_or_lib(file) {
        return None;
    }
    let dir = file.parent()?;
    let flat = dir.join(format!("{name}.rs"));
    if flat.is_file() {
        return Some(flat);
    }
    let nested = dir.join(name).join("mod.rs");
    nested.is_file().then_some(nested)
}
