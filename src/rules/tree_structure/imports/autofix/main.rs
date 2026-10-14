use {
    super::{insert, leaf, remove, render, t::Removal},
    crate::rules::c::{path, suggestions},
    std::{collections::HashMap, fs, ops::Range, path::Path},
    syn::spanned::Spanned,
};

type Edit = (Range<usize>, String);

pub fn file(path: &Path, rewrites: &HashMap<Vec<String>, Vec<String>>) -> Option<String> {
    let source = fs::read_to_string(path).expect("failed to read file");
    let ast = syn::parse_file(&source).expect("failed to parse file");
    let mut moved = Vec::new();
    let edits: Vec<Edit> = ast
        .items
        .iter()
        .filter_map(|item| _edit(&source, item, rewrites, &mut moved))
        .collect();
    if edits.is_empty() {
        return None;
    }
    fs::write(path, _applied(source, edits)).expect("failed to write file");
    Some(format!("{} ({})", path::rel(path), suggestions::shortlist(&moved)))
}

fn _edit(
    source: &str,
    item: &syn::Item,
    rewrites: &HashMap<Vec<String>, Vec<String>>,
    moved: &mut Vec<String>,
) -> Option<Edit> {
    let syn::Item::Use(item) = item else {
        return None;
    };
    let mut removal = Removal {
        rewrites,
        moved: Vec::new(),
    };
    let mut tree = remove::fixable(&item.tree, &[], &mut removal);
    if removal.moved.is_empty() {
        return None;
    }
    for m in removal.moved {
        moved.push(render::flat(&leaf::chain(&m.path, m.leaf.clone())));
        tree = Some(insert::at(tree, &m.path, m.leaf));
    }
    let start = _offset(source) + item.tree.span().byte_range().start;
    let end = _offset(source) + item.semi_token.span().byte_range().start;
    let (indent, used) = _position(source, start);
    Some((start..end, render::tree(&tree?, indent, used)))
}

// not-obvious: syn skips a leading byte-order mark, so its offsets start after it.
fn _offset(source: &str) -> usize {
    source.len() - source.trim_start_matches('\u{feff}').len()
}

fn _position(source: &str, start: usize) -> (usize, usize) {
    let line_start = source[..start].rfind('\n').map_or(0, |i| i + 1);
    let line = &source[line_start..start];
    (line.len() - line.trim_start().len(), line.chars().count())
}

fn _applied(mut source: String, mut edits: Vec<Edit>) -> String {
    let newline = if source.contains("\r\n") { "\r\n" } else { "\n" };
    edits.sort_by_key(|(range, _)| std::cmp::Reverse(range.start));
    for (range, text) in edits {
        source.replace_range(range, &text.replace('\n', newline));
    }
    source
}
