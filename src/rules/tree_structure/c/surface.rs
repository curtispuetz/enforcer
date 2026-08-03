use {
    super::{children, dedup, leaves, modules, path, reexports},
    crate::rules::{c::files, tree_structure::t::SurfaceItem},
    std::{
        collections::HashSet,
        path::{Path, PathBuf},
    },
    syn::UseTree,
};

type Visited = HashSet<(PathBuf, Vec<String>)>;

pub fn items(module: &Path) -> Vec<SurfaceItem> {
    let root = modules::root_file(module);
    let Some(segments) = path::module(&root) else {
        return Vec::new();
    };
    dedup::unique(_exports(&root, &segments, &mut Visited::new()))
}

fn _exports(file: &Path, module: &[String], visited: &mut Visited) -> Vec<SurfaceItem> {
    if !file.is_file() || !visited.insert((file.to_path_buf(), module.to_vec())) {
        return Vec::new();
    }
    let ast = files::ast_parse(file);
    let mut ret = leaves::of(&ast, file);
    for (name, child) in children::public(file, &ast) {
        let mut deeper = module.to_vec();
        deeper.push(name);
        ret.extend(_exports(&child, &deeper, visited));
    }
    for tree in reexports::trees(&ast) {
        _reexported(tree, file, module, visited, &mut ret);
    }
    ret
}

fn _reexported(
    tree: &UseTree,
    file: &Path,
    module: &[String],
    visited: &mut Visited,
    out: &mut Vec<SurfaceItem>,
) {
    match tree {
        UseTree::Path(p) if p.ident == "self" => {
            _reexported(&p.tree, file, module, visited, out);
        }
        UseTree::Path(p) => _descend(p, file, module, visited, out),
        UseTree::Glob(_) => out.extend(_exports(file, module, visited)),
        UseTree::Name(n) => {
            _named(&n.ident.to_string(), file, module, visited, out);
        }
        UseTree::Group(g) => {
            for item in &g.items {
                _reexported(item, file, module, visited, out);
            }
        }
        UseTree::Rename(_) => {}
    }
}

fn _descend(
    p: &syn::UsePath,
    file: &Path,
    module: &[String],
    visited: &mut Visited,
    out: &mut Vec<SurfaceItem>,
) {
    if let Some(child) = children::find(file, &p.ident.to_string()) {
        _reexported(&p.tree, &child, module, visited, out);
    }
}

fn _named(
    name: &str,
    file: &Path,
    module: &[String],
    visited: &mut Visited,
    out: &mut Vec<SurfaceItem>,
) {
    let exports = _exports(file, module, visited);
    out.extend(exports.into_iter().filter(|i| i.name == name));
}
