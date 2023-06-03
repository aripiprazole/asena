use std::fmt::Debug;

use crate::ast::node::{Child, Tree, TreeKind};
use crate::lexer::span::{Loc, Spanned};
use crate::report::Report;

use super::error::ParseError;
use super::Parser;

#[derive(Clone)]
pub enum Event {
    Open(Spanned<TreeKind>),
    Field(&'static str),
    Close,
    Advance,
}

pub struct MarkOpened(usize, Loc);

pub struct MarkClosed(usize, Loc);

pub struct RedTree {
    pub data: Spanned<Tree>,
    pub report: Report<ParseError>,
}

impl<'a> Parser<'a> {
    pub fn build_tree(self) -> RedTree {
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
                    stack.push(kind.replace(Tree::new(kind.value)));
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
                        .push(token.replace(Child::Token(token.value().clone())))
                }

                Event::Field(name) => {
                    let last_item = stack.last_mut().unwrap();
                    let last_child = last_item.children.last_mut().unwrap();
                    match &mut last_child.value {
                        Child::Tree(tree) => tree.name = Some(name),
                        Child::Token(token) => token.name = Some(name),
                    }
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

        let tree = stack.pop().unwrap();
        let mut report = Report::new(self.source, tree.clone());
        for diagnostic in &self.errors {
            report.diagnostics.push(diagnostic.clone());
        }

        RedTree { data: tree, report }
    }
}

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

impl MarkClosed {
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

impl RedTree {
    pub fn unwrap(mut self) -> Spanned<Tree> {
        if self.has_errors() {
            self.report.dump();

            panic!("Called `RedTree::unwrap` on a failed-to-parse tree");
        }

        self.data
    }

    pub fn has_errors(&self) -> bool {
        !self.report.diagnostics.is_empty()
    }

    pub fn data(&self) -> &Spanned<Tree> {
        &self.data
    }

    pub fn report(&self) -> &Report<ParseError> {
        &self.report
    }
}

impl Debug for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Open(token) => write!(f, "Open({token:#?})"),
            Self::Field(name) => write!(f, "Field({name:#?})"),
            Self::Advance => write!(f, "Advance"),
            Self::Close => write!(f, "Close"),
        }
    }
}

impl From<RedTree> for Spanned<Tree> {
    fn from(value: RedTree) -> Self {
        value.data
    }
}

impl From<RedTree> for Report<ParseError> {
    fn from(value: RedTree) -> Self {
        value.report
    }
}
