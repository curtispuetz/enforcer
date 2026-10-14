use {
    super::{exports, files, path, reexports, use_paths},
    crate::rules::t::{ExportTables, FileImports},
    std::path::{Path, PathBuf},
};

pub fn build() -> Vec<FileImports> {
    let parsed: Vec<(PathBuf, syn::File)> = files::all()
        .into_iter()
        .map(|file| {
            let ast = files::ast_parse(&file);
            (file, ast)
        })
        .collect();
    let tables = _tables(&parsed);
    parsed
        .iter()
        .filter_map(|(file, ast)| _file(file, ast, &tables))
        .collect()
}

fn _tables(parsed: &[(PathBuf, syn::File)]) -> ExportTables {
    parsed
        .iter()
        .filter_map(|(file, ast)| {
            Some((path::module(file)?, exports::table(file, ast)))
        })
        .collect()
}

fn _file(file: &Path, ast: &syn::File, tables: &ExportTables) -> Option<FileImports> {
    let targets: Vec<Vec<String>> = use_paths::internal(ast)
        .iter()
        .filter_map(|use_path| path::absolute(use_path, file))
        .collect();
    let resolved = targets
        .iter()
        .map(|target| reexports::follow(target, tables))
        .collect();
    Some(FileImports {
        path: file.to_path_buf(),
        dir: path::file_dir(file)?,
        module: path::module(file)?,
        targets,
        resolved,
    })
}
