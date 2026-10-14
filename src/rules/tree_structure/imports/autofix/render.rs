use super::t::Tree;

const MAX_WIDTH: usize = 88;
const INDENT: usize = 4;

pub fn flat(tree: &Tree) -> String {
    match tree {
        Tree::Path(seg, sub) => format!("{seg}::{}", flat(sub)),
        Tree::Name(name) => name.clone(),
        Tree::Rename(name, alias) => format!("{name} as {alias}"),
        Tree::Glob => "*".to_string(),
        Tree::Group(items) => format!("{{{}}}", _joined(items)),
    }
}

// not-obvious: mirrors rustfmt's nested-import layout: a list stays on one line if
// it fits and holds no multi-item sublist; otherwise each path item gets its own
// line and plain names are packed together.
pub fn tree(tree: &Tree, indent: usize, used: usize) -> String {
    match tree {
        Tree::Path(seg, sub) => {
            format!("{seg}::{}", self::tree(sub, indent, used + seg.len() + 2))
        }
        Tree::Group(items) => _group(items, indent, used),
        leaf => flat(leaf),
    }
}

fn _group(items: &[Tree], indent: usize, used: usize) -> String {
    let one_line = format!("{{{}}}", _joined(items));
    if !items.iter().any(_ends_in_list) && used + one_line.len() < MAX_WIDTH {
        return one_line;
    }
    let inner = indent + INDENT;
    let lines: Vec<String> = _lines(items, inner)
        .iter()
        .map(|line| format!("{}{line}", " ".repeat(inner)))
        .collect();
    format!("{{\n{}\n{}}}", lines.join("\n"), " ".repeat(indent))
}

fn _lines(items: &[Tree], inner: usize) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    let mut packable = false;
    for item in items {
        let text = format!("{},", tree(item, inner, inner));
        let simple = !text.contains("::") && !text.contains('\n');
        match lines.last_mut() {
            Some(line) if packable && simple && inner + line.len() + text.len() < MAX_WIDTH => {
                line.push(' ');
                line.push_str(&text);
            }
            _ => lines.push(text),
        }
        packable = simple;
    }
    lines
}

fn _ends_in_list(tree: &Tree) -> bool {
    match tree {
        Tree::Path(_, sub) => _ends_in_list(sub),
        Tree::Group(items) => items.len() > 1,
        _ => false,
    }
}

fn _joined(items: &[Tree]) -> String {
    items.iter().map(flat).collect::<Vec<String>>().join(", ")
}
