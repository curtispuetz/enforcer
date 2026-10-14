use super::t::Tree;

pub fn is_self(tree: &Tree) -> bool {
    matches!(tree, Tree::Name(name) | Tree::Rename(name, _) if name == "self")
}

// not-obvious: `self` is only valid inside braces, so a moved `self` keeps them.
pub fn chain(path: &[String], leaf: Tree) -> Tree {
    let base = if is_self(&leaf) && !path.is_empty() {
        Tree::Group(vec![leaf])
    } else {
        leaf
    };
    path.iter()
        .rev()
        .fold(base, |sub, seg| Tree::Path(seg.clone(), Box::new(sub)))
}
