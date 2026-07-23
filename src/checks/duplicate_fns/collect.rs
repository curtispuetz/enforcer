use syn::Item;

use crate::{
    c::{files, path},
    s::EXISTING_SRC_DIRS,
};

use super::t::FreeFn;

// Gather every free (module-level) function across the source tree. Methods on
// impl/trait blocks are intentionally excluded — the check is about free
// functions only.
pub fn all() -> Vec<FreeFn> {
    let mut found = Vec::new();
    for dir_name in EXISTING_SRC_DIRS.iter() {
        for file_path in files::rs(dir_name) {
            let file = files::parse(&file_path);
            let relative = path::rel(&file_path);
            _items(&file.items, &relative, &mut found);
        }
    }
    found
}

fn _items(items: &[Item], relative: &str, found: &mut Vec<FreeFn>) {
    for item in items {
        match item {
            Item::Fn(function) => found.push(FreeFn {
                path: relative.to_string(),
                name: function.sig.ident.to_string(),
                line: function.sig.ident.span().start().line,
                item: function.clone(),
            }),
            Item::Mod(module) => {
                if let Some((_, items)) = &module.content {
                    _items(items, relative, found);
                }
            }
            _ => {}
        }
    }
}
