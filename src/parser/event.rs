use crate::ast::node::{Child, Tree, TreeKind};
use crate::lexer::span::{Loc, Spanned};

use super::Parser;

#[derive(Debug, Clone)]
pub enum Event {
    Open(Spanned<TreeKind>),
    Close,
    Advance,
}

pub struct MarkOpened(usize, Loc);

impl MarkOpened {
    pub fn new(index: usize, loc: Loc) -> Self {
        Self(index, loc)
    }

    pub fn index(&self) -> usize {
        self.0
    }

    pub fn span(&self) -> Loc {
        self.1.clone()
    }
}

impl<'a> Parser<'a> {
    pub fn build_tree(self) -> Spanned<Tree> {
        let mut tokens = self.tokens.into_iter();
        let mut events = self.events;
        let mut stack = vec![];

        // Special case: pop the last `Close` event to ensure
        // that the stack is non-empty inside the loop.
        assert!(matches!(events.pop(), Some(Event::Close)));

        for event in events {
            match event {
                // Starting a new node; just push an empty tree to the stack.
                Event::Open(kind) => {
                    stack.push(kind.replace(Tree::new(kind.value.clone())));
                }

                // A tree is done.
                // Pop it off the stack and append to a new current tree.
                Event::Close => {
                    let tree = stack.pop().expect("Stack should be not empty");

                    stack
                        .last_mut()
                        // If we don't pop the last `Close` before this loop,
                        // this unwrap would trigger for it.
                        .unwrap()
                        .value
                        .children
                        .push(tree.replace(Child::Tree(tree.value.clone())))
                }

                // Consume a token and append it to the current tree
                Event::Advance => {
                    let token = tokens.next().expect("Stack should have next element");

                    stack
                        .last_mut()
                        .unwrap()
                        .value
                        .children
                        .push(token.replace(Child::Token(token.value.clone())))
                }
            }
        }

        // Our parser will guarantee that all the trees are closed
        // and cover the entirety of tokens.

        assert!(
            stack.len() == 1,
            "The stack should contain just the tree element"
        );

        assert!(
            tokens.next().is_none(),
            "The token stream still contain something"
        );

        stack.pop().unwrap()
    }
}