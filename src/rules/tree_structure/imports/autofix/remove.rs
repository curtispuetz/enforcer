use {
    super::{
        leaf,
        t::{Moved, Removal, Tree},
    },
    syn::{UseGroup, UseTree},
};

pub fn fixable(tree: &UseTree, prefix: &[String], removal: &mut Removal) -> Option<Tree> {
    match tree {
        UseTree::Path(p) => {
            let seg = p.ident.to_string();
            let sub = fixable(&p.tree, &[prefix, std::slice::from_ref(&seg)].concat(), removal)?;
            Some(Tree::Path(seg, Box::new(sub)))
        }
        UseTree::Name(n) => {
            let name = n.ident.to_string();
            _take(prefix, Some(&name), Tree::Name(name.clone()), removal)
        }
        UseTree::Rename(r) => {
            let (name, alias) = (r.ident.to_string(), r.rename.to_string());
            _take(prefix, Some(&name), Tree::Rename(name.clone(), alias), removal)
        }
        UseTree::Glob(_) => _take(prefix, None, Tree::Glob, removal),
        UseTree::Group(g) => _group(g, prefix, removal),
    }
}

fn _take(
    prefix: &[String],
    name: Option<&String>,
    leaf: Tree,
    removal: &mut Removal,
) -> Option<Tree> {
    let key: Vec<String> = prefix.iter().chain(name).cloned().collect();
    let Some(new_path) = removal.rewrites.get(&key) else {
        return Some(leaf);
    };
    let path = new_path[..new_path.len() - usize::from(name.is_some())].to_vec();
    removal.moved.push(Moved { path, leaf });
    None
}

fn _group(group: &UseGroup, prefix: &[String], removal: &mut Removal) -> Option<Tree> {
    let mut kept: Vec<Tree> = group
        .items
        .iter()
        .filter_map(|item| fixable(item, prefix, removal))
        .collect();
    let unchanged = kept.len() == group.items.len();
    match kept.len() {
        0 => None,
        1 if !unchanged && !leaf::is_self(&kept[0]) => kept.pop(),
        _ => Some(Tree::Group(kept)),
    }
}
