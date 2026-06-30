use std::{fs, path::Path};
use syn::UseTree;

// The rules behind these tests are:
// 1. a file can only import from its own directory or sub directories
// 2. Exception to rule 1 is that files can import from c directories if the prefix of
//    that c directory path is in the files prefix
// 3. Rules 1 and 2 also apply within a c directory as if that c directory where its
//    own root directory project.
// 4. The rules don't apply inside 's' (static data) and 't' (types) directories, nor
//    when any other file imports from these directories.

pub fn check_dir(root: &Path, dir_name: &str) -> usize {
    let dir = root.join(dir_name);
    let pattern = format!("{}/**/*.rs", dir.to_string_lossy().replace('\\', "/"));
    let mut total = 0;
    for path in glob::glob(&pattern)
        .expect("invalid glob")
        .filter_map(|p| p.ok())
    {
        total += _check_file(&path, &dir);
    }
    total
}

fn _check_file(path: &Path, src_dir: &Path) -> usize {
    let file_dir = match _file_dir_segments(path, src_dir) {
        Some(d) => d,
        None => return 0,
    };
    if file_dir.iter().any(|s| _is_s_or_t(s)) {
        return 0;
    }
    let source = fs::read_to_string(path).expect("failed to read file");
    let file = syn::parse_file(&source).expect("failed to parse file");
    let mut violations: Vec<String> = Vec::new();

    for item in &file.items {
        if let syn::Item::Use(u) = item {
            for use_path in _expand_use_tree(vec![], &u.tree) {
                if use_path.first().map(|s| s.as_str()) != Some("crate") {
                    continue;
                }
                if !_is_import_allowed(&use_path, &file_dir) {
                    violations.push(format!("  use {};", use_path.join("::")));
                }
            }
        }
    }

    if !violations.is_empty() {
        eprintln!("{} has disallowed imports:", path.display());
        for v in &violations {
            eprintln!("{v}");
        }
    }
    violations.len()
}

fn _file_dir_segments(path: &Path, src_dir: &Path) -> Option<Vec<String>> {
    let rel = path.strip_prefix(src_dir).ok()?;
    let mut segments = vec!["crate".to_string()];
    for component in rel.parent()?.components() {
        if let std::path::Component::Normal(s) = component {
            segments.push(s.to_str()?.to_string());
        }
    }
    Some(segments)
}

fn _expand_use_tree(prefix: Vec<String>, tree: &UseTree) -> Vec<Vec<String>> {
    match tree {
        UseTree::Path(p) => {
            let mut new_prefix = prefix;
            new_prefix.push(p.ident.to_string());
            _expand_use_tree(new_prefix, &p.tree)
        }
        UseTree::Name(n) => {
            let mut path = prefix;
            path.push(n.ident.to_string());
            vec![path]
        }
        UseTree::Rename(r) => {
            let mut path = prefix;
            path.push(r.ident.to_string());
            vec![path]
        }
        UseTree::Glob(_) => vec![prefix],
        UseTree::Group(g) => g
            .items
            .iter()
            .flat_map(|item| _expand_use_tree(prefix.clone(), item))
            .collect(),
    }
}

fn _is_s_or_t(seg: &str) -> bool {
    seg == "s" || seg == "t"
}

fn _is_import_allowed(use_path: &[String], file_dir: &[String]) -> bool {
    if use_path.iter().any(|s| _is_s_or_t(s)) {
        return true;
    }
    if use_path.starts_with(file_dir) {
        return true;
    }
    for (i, seg) in use_path.iter().enumerate() {
        if seg == "c" && file_dir.starts_with(&use_path[..i]) {
            if file_dir.get(i).map(|s| s.as_str()) == Some("c") {
                return _is_import_allowed(&use_path[i + 1..], &file_dir[i + 1..]);
            }
            return true;
        }
    }
    false
}
