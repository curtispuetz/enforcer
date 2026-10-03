use {
    super::t::{Folder, Member},
    crate::rules::{
        c::path as path1,
        s::COMMON,
        tree_structure::c::{modules, path},
    },
    std::{fs, path::Path},
};

pub fn all() -> Vec<Folder> {
    modules::common()
        .iter()
        .filter(|p| p.is_dir() && p.file_name().is_some_and(|n| n == "c"))
        .filter_map(|p| _folder(p))
        .collect()
}

fn _folder(dir: &Path) -> Option<Folder> {
    let segments = path::module(&dir.join("mod.rs"))?;
    let (_, parent) = segments.split_last()?;
    let members = fs::read_dir(dir)
        .ok()?
        .flatten()
        .filter_map(|entry| _member(&entry.path(), &segments))
        .collect();
    Some(Folder {
        parent: parent.to_vec(),
        segments,
        members,
    })
}

fn _member(path: &Path, folder: &[String]) -> Option<Member> {
    let name = _module_name(path)?;
    if COMMON.contains(&name.as_str()) || path1::ignored(path) {
        return None;
    }
    let mut segments = folder.to_vec();
    segments.push(name.clone());
    Some(Member {
        name,
        path: path.to_path_buf(),
        segments,
    })
}

fn _module_name(path: &Path) -> Option<String> {
    let is_module = if path.is_dir() {
        path.join("mod.rs").is_file()
    } else {
        path.extension().is_some_and(|e| e == "rs") && !path1::is_mod_or_lib(path)
    };
    let stem = path.file_stem()?.to_str()?;
    is_module.then(|| stem.to_string())
}
