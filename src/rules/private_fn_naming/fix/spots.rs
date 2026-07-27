use syn::visit::Visit;

use crate::rules::private_fn_naming::t::{Finder, Misnamed};

use super::macros;

pub fn of_names(
    file: &syn::File,
    names: &Misnamed,
) -> (Vec<(usize, usize)>, Vec<String>) {
    let mut finder = Finder {
        names,
        spots: Vec::new(),
        bound: Vec::new(),
        macros: Vec::new(),
    };
    finder.visit_file(file);
    let found = macros::spots(finder.macros, names);
    _filtered([finder.spots, found].concat(), &finder.bound)
}

// not-obvious: a name also used as a binding in the file is left alone, since
// renaming its uses without renaming the binding would break the code.
fn _filtered(
    spots: Vec<(String, usize, usize)>,
    bound: &[String],
) -> (Vec<(usize, usize)>, Vec<String>) {
    let mut out = Vec::new();
    let mut renamed: Vec<String> = Vec::new();
    for (name, line, col) in spots {
        if bound.contains(&name) {
            continue;
        }
        out.push((line, col));
        if !renamed.contains(&name) {
            renamed.push(name);
        }
    }
    (out, renamed)
}
