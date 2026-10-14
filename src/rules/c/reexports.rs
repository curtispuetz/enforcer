use crate::rules::t::{ExportTables, ModuleExports};

const MAX_HOPS: usize = 16;

pub fn follow(target: &[String], tables: &ExportTables) -> Vec<String> {
    let mut path = target.to_vec();
    if path.last().is_some_and(|seg| seg == "self") {
        path.pop();
    }
    for _ in 0..MAX_HOPS {
        match _hop(&path, tables) {
            Some(next) => path = next,
            None => return path,
        }
    }
    target.to_vec()
}

fn _hop(path: &[String], tables: &ExportTables) -> Option<Vec<String>> {
    let depth = _first_reexported(path, tables)?;
    let mut ret = _source(&tables[&path[..depth]], &path[depth], tables)?;
    ret.extend_from_slice(&path[depth + 1..]);
    Some(ret)
}

// not-obvious: walks down through declared modules, stopping at the first segment
// its module neither declares as a child nor defines, since only a re-export
// can supply it.
fn _first_reexported(path: &[String], tables: &ExportTables) -> Option<usize> {
    for depth in 1..path.len() {
        let exports = tables.get(&path[..depth])?;
        if !exports.children.contains(&path[depth]) {
            return (!exports.items.contains(&path[depth])).then_some(depth);
        }
    }
    None
}

fn _source(
    exports: &ModuleExports,
    name: &str,
    tables: &ExportTables,
) -> Option<Vec<String>> {
    if let Some(target) = exports.named.get(name) {
        return Some(target.clone());
    }
    let globs: Vec<&Vec<String>> = exports.globs.iter().collect();
    let declaring: Vec<&Vec<String>> = globs
        .iter()
        .copied()
        .filter(|glob| _declares(tables, glob, name))
        .collect();
    let source = match (declaring.as_slice(), globs.as_slice()) {
        ([one], _) | ([], [one]) => one,
        _ => return None,
    };
    let mut ret = source.to_vec();
    ret.push(name.to_string());
    Some(ret)
}

fn _declares(tables: &ExportTables, module: &[String], name: &str) -> bool {
    tables.get(module).is_some_and(|exports| {
        exports.children.contains(name)
            || exports.items.contains(name)
            || exports.named.contains_key(name)
    })
}
