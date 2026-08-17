use {
    crate::s::{FILE_CONFIG, ROOT},
    std::{collections::HashSet, path::Path},
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
