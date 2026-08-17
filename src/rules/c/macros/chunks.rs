use {
    proc_macro2::{TokenStream, TokenTree},
    syn::Expr,
};

pub fn exprs(tokens: &TokenStream) -> Vec<Expr> {
    let mut ret = Vec::new();
    for chunk in _split(tokens) {
        match syn::parse2::<Expr>(chunk.clone()) {
            Ok(expr) => ret.push(expr),
            Err(_) => ret.extend(_in_groups(&chunk)),
        }
    }
    ret
}

fn _in_groups(chunk: &TokenStream) -> Vec<Expr> {
    let mut ret = Vec::new();
    for tree in chunk.clone() {
        if let TokenTree::Group(group) = tree {
            ret.extend(exprs(&group.stream()));
        }
    }
    ret
}

fn _split(tokens: &TokenStream) -> Vec<TokenStream> {
    let mut ret = Vec::new();
    let mut current: Vec<TokenTree> = Vec::new();
    for tree in tokens.clone() {
        if _is_separator(&tree) {
            ret.push(current.drain(..).collect());
        } else {
            current.push(tree);
        }
    }
    ret.push(current.into_iter().collect());
    ret
}

fn _is_separator(tree: &TokenTree) -> bool {
    matches!(tree, TokenTree::Punct(p) if matches!(p.as_char(), ',' | ';'))
}
