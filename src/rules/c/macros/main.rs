use {
    super::chunks,
    proc_macro2::TokenStream,
    syn::{Block, Expr, ExprBlock, Token, parse::Parser, punctuated::Punctuated},
};

pub fn exprs(tokens: &TokenStream) -> Vec<Expr> {
    _comma_separated(tokens)
        .or_else(|| _statements(tokens))
        .unwrap_or_else(|| chunks::exprs(tokens))
}

fn _comma_separated(tokens: &TokenStream) -> Option<Vec<Expr>> {
    let parser = Punctuated::<Expr, Token![,]>::parse_terminated;
    let parsed = parser.parse2(tokens.clone()).ok()?;
    Some(parsed.into_iter().collect())
}

fn _statements(tokens: &TokenStream) -> Option<Vec<Expr>> {
    let stmts = Block::parse_within.parse2(tokens.clone()).ok()?;
    let block = Block {
        brace_token: syn::token::Brace::default(),
        stmts,
    };
    let wrapped = ExprBlock {
        attrs: Vec::new(),
        label: None,
        block,
    };
    Some(vec![Expr::Block(wrapped)])
}
