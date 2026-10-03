use {
    super::t::FileImports,
    crate::rules::{
        c::files,
        tree_structure::c::{path, use_paths},
    },
    std::path::PathBuf,
};

pub fn build() -> Vec<FileImports> {
    files::all().into_iter().filter_map(_file).collect()
}

fn _file(path: PathBuf) -> Option<FileImports> {
    let dir = path::file_dir(&path)?;
    let module = path::module(&path)?;
    let targets = use_paths::internal(&files::ast_parse(&path))
        .iter()
        .filter_map(|use_path| path::absolute(use_path, &path))
        .collect();
    Some(FileImports {
        path,
        dir,
        module,
        targets,
    })
}
