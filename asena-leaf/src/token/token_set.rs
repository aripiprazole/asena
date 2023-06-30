use asena_span::Spanned;

use crate::{
    node::{Child, Tree},
    token::Token,
};

pub trait HasTokens {
    fn tokens(&self) -> Vec<Spanned<Token>>;
}

impl HasTokens for Tree {
    fn tokens(&self) -> Vec<Spanned<Token>> {
        let mut buf = Vec::new();
        for child in self.children.iter() {
            match child.value {
                Child::Tree(ref tree) => {
                    buf.extend(tree.tokens());
                }
                Child::Token(ref token) => {
                    buf.push(child.replace(token.clone()));
                }
            }
        }
        buf
    }
}

impl<T: HasTokens> HasTokens for Spanned<T> {
    fn tokens(&self) -> Vec<Spanned<Token>> {
        self.value.tokens()
    }
}

impl HasTokens for Spanned<Token> {
    fn tokens(&self) -> Vec<Spanned<Token>> {
        vec![self.clone()]
    }
}
