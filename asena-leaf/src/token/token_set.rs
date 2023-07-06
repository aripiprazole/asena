use asena_interner::Intern;
use asena_span::Spanned;

use crate::{
    node::{Child, Tree},
    token::Token,
};

pub trait HasTokens {
    fn tokens(&self) -> Vec<Intern<Spanned<Token>>>;
}

impl HasTokens for Tree {
    fn tokens(&self) -> Vec<Intern<Spanned<Token>>> {
        let mut buf = Vec::new();
        for child in self.children.iter() {
            match child {
                Child::Tree(ref tree) => {
                    buf.extend(tree.tokens());
                }
                Child::Token(ref token) => {
                    buf.push(token.clone());
                }
            }
        }
        buf
    }
}

impl<T: HasTokens> HasTokens for Spanned<T> {
    fn tokens(&self) -> Vec<Intern<Spanned<Token>>> {
        self.value.tokens()
    }
}

impl HasTokens for Intern<Spanned<Token>> {
    fn tokens(&self) -> Vec<Intern<Spanned<Token>>> {
        vec![self.clone()]
    }
}
