use {
    crate::rules::private_fn_naming::t::Misnamed,
    proc_macro2::{Spacing, TokenStream, TokenTree},
};

pub fn spots(
    streams: Vec<TokenStream>,
    names: &Misnamed,
) -> Vec<(String, usize, usize)> {
    let mut found = Vec::new();
    for stream in streams {
        _walk(stream, names, &mut found);
    }
    found
}

fn _walk(
    stream: TokenStream,
    names: &Misnamed,
    found: &mut Vec<(String, usize, usize)>,
) {
    let trees: Vec<TokenTree> = stream.into_iter().collect();
    for (i, tree) in trees.iter().enumerate() {
        match tree {
            TokenTree::Ident(_) => _push(&trees, i, names, found),
            TokenTree::Group(group) => _walk(group.stream(), names, found),
            _ => {}
        }
    }
}

fn _push(
    trees: &[TokenTree],
    i: usize,
    names: &Misnamed,
    found: &mut Vec<(String, usize, usize)>,
) {
    let TokenTree::Ident(ident) = &trees[i] else {
        return;
    };
    let name = ident.to_string();
    if _field_like(trees, i) || !_named(names, &name, _after_dot(trees, i)) {
        return;
    }
    let start = ident.span().start();
    found.push((name, start.line, start.column));
}

// not-obvious: an ident after a `.` is a field or method access, and one
// before a lone `:` is a struct-literal field, neither of which is a call.
fn _named(names: &Misnamed, name: &String, after_dot: bool) -> bool {
    if after_dot {
        return names.methods.contains(name);
    }
    names.free.contains(name) || names.methods.contains(name)
}

fn _after_dot(trees: &[TokenTree], i: usize) -> bool {
    matches!(
        trees.get(i.wrapping_sub(1)),
        Some(TokenTree::Punct(p)) if p.as_char() == '.'
    )
}

fn _field_like(trees: &[TokenTree], i: usize) -> bool {
    matches!(
        trees.get(i + 1),
        Some(TokenTree::Punct(p))
            if p.as_char() == ':' && p.spacing() == Spacing::Alone
    )
}
