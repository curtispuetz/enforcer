use super::{leaf, render, t::Tree};

pub fn at(tree: Option<Tree>, path: &[String], new: Tree) -> Tree {
    let Some(tree) = tree else {
        return leaf::chain(path, new);
    };
    match (tree, path.split_first()) {
        (Tree::Group(items), _) => _into_group(items, path, new),
        (Tree::Path(seg, sub), Some((first, rest))) if seg == *first => {
            Tree::Path(seg, Box::new(at(Some(*sub), rest, new)))
        }
        (other, _) => _into_group(vec![other], path, new),
    }
}

fn _into_group(mut items: Vec<Tree>, path: &[String], new: Tree) -> Tree {
    let same_root = path.first().and_then(|first| {
        items
            .iter()
            .position(|item| matches!(item, Tree::Path(seg, _) if seg == first))
    });
    if let Some(i) = same_root {
        let item = items.remove(i);
        items.insert(i, at(Some(item), path, new));
        return Tree::Group(items);
    }
    let new = leaf::chain(path, new);
    let key = _key(&new);
    let at = items
        .iter()
        .position(|item| _key(item) > key)
        .unwrap_or(items.len());
    items.insert(at, new);
    Tree::Group(items)
}

// not-obvious: rustfmt orders `self`, `super` and `crate` first, then the rest
// in ASCII order.
fn _key(tree: &Tree) -> (usize, String) {
    let text = render::flat(tree);
    let first = text.split("::").next().unwrap_or_default();
    let rank = ["self", "super", "crate"].iter().position(|r| *r == first);
    (rank.unwrap_or(3), text)
}
